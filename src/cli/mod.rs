use anyhow::{Context, Result};
use chrono::NaiveDate;
use chrono_tz::Tz;
use clap::{ArgAction, Parser, Subcommand};
use colored::*;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use std::io::Write;

use crate::api;
use crate::datetime;
use crate::times::types;

pub const ALLOWED_TIMES: [&'static str; 8] = [
    "fajr", "sunrise", "dhuhr", "asr", "maghrib", "isha", "midnight", "fardh",
];

pub const TIMES_DESC: [&'static str; 8] = [
    "The dawn prayer time.",
    "Sunrise time. Fajr ends at sunrise",
    "The mid-day prayer time.",
    "The evening prayer time.",
    "The sunset prayer time.",
    "The night prayer time.",
    "Islamic midnight time. Isha ends at midnight",
    "Gets only the 5 obligatory (fardh) prayer times. Ignores any others",
];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Use location (city/country) to get prayer times. WARNING: Uses external API call, network connection required.
    Location {
        #[command(flatten)]
        common: CommonConfig,

        /// City to calculate the times for
        #[arg(long)]
        city: String,

        /// Country to calculate the times for
        #[arg(long)]
        country: String,
    },
    /// Use coordinates (latitude/longitude) to get prayer times.
    Coord {
        #[command(flatten)]
        common: CommonConfig,

        /// Latitude to calculate the times for
        #[arg(long)]
        lat: f64,

        /// Longitude to calculate the time for
        #[arg(long)]
        lng: f64,
    },
    /// Lists all the available timings.
    Timings,
    /// Lists all the calculation authorities
    Authority,
}

#[derive(Parser, Debug)]
pub struct CommonConfig {
    /// Names of the timings to calculate for (see `salah timings` for available values) (ignored by --all)
    #[arg(action=ArgAction::Append)]
    timings: Vec<String>,

    /// Date to calculate the timings for (YYYY-MM-DD). Use `today` for today's date.
    #[arg(short, long, default_value_t=String::from("today"))]
    date: String,

    /// Timezone to output the timings for.
    #[arg(short, long, default_value_t=String::from("America/Toronto"))]
    timezone: String,

    /// Calculates all the available prayer timings.
    #[arg(short, long, action=ArgAction::SetTrue)]
    all: bool,

    /// If set, uses Hanafi madhab for Asr calculation [default: false]
    #[arg(long, action=ArgAction::SetTrue)]
    hanafi: bool,

    /// Calculation authority to use (see `salah authority` for available values)
    #[arg(long, default_value_t=String::from("ISNA"))]
    auth: String,

    /// Format string for timings output. See `man strftime` for configuration.
    #[arg(long, default_value_t=String::from("%H:%M:%S"))]
    format: String,
} 

impl CommonConfig {
    fn parsed_date(&self) -> Result<NaiveDate> {
        let tz: Tz = self
            .parsed_timezone()
            .with_context(|| format!("Unable to parse timezone"))?;

        return datetime::str2date(&self.date, tz);
    }

    fn parsed_timezone(&self) -> Result<Tz> {
        match self.timezone.parse::<Tz>() {
            Ok(t) => Ok(t),
            Err(e) => return Err(anyhow::anyhow!(e)),
        }
    }

    fn parsed_timings(&self) -> Result<Vec<types::Timing>> {
        let mut timings: Vec<types::Timing> = vec![];
        let all_timings: Vec<types::Timing> = types::Timing::list().into_iter().collect();
        if self.all {
            timings = all_timings;
        } else {
            for timing in &self.timings {
                let m = match types::Timing::from_str(timing) {
                    Some(t) => t,
                    None => return Err(anyhow::anyhow!("timing = `{}` is not valid!", timing)),
                };
                timings.push(m);
            }
        }

        return Ok(timings);
    }

    fn parsed_auth(&self) -> Result<types::Authority> {
        match types::Authority::from_str(&self.auth) {
            Some(a) => Ok(a),
            None => Err(anyhow::anyhow!("authority = `{}` is not valid!", self.auth)),
        }
    }
}

#[derive(Debug)]
pub enum ParsedOptions {
    Calculation {
        date: NaiveDate,
        timezone: Tz,
        lat: f64,
        lng: f64,
        timings: Vec<types::Timing>,
        auth: types::Authority,
        school: types::School,
        format: String
    },
    Timings,
    Authority,
}

/// Validates the command-line arguments
pub async fn parse() -> Result<ParsedOptions> {
    let opts = Options::parse();

    match &opts.commands {
        Commands::Location {
            common,
            city,
            country,
        } => {
            let date = common
                .parsed_date()
                .with_context(|| format!("Failed to create date with `{}`", common.date))?;
            let timezone = common
                .parsed_timezone()
                .with_context(|| format!("Failed to create timezone with `{}`", common.timezone))?;
            let timings = common
                .parsed_timings()
                .with_context(|| format!("Failed to parse timings with {:?}", common.timings))?;
            let auth = common
                .parsed_auth()
                .with_context(|| format!("Failed to parse authority with `{}`", common.auth))?;
            let school = if common.hanafi {
                types::School::Hanafi
            } else {
                types::School::Shafi
            };
            let format = common.format.to_owned();

            // API call to get lat,lng from city, country
            #[derive(Deserialize)]
            struct APICoord {
                lat: String,
                lon: String,
            }
            let url = format!(
                "https://nominatim.openstreetmap.org/search?city={}&country={}&format=jsonv2",
                city, country
            );
            let mut headers = HeaderMap::new();
            headers.insert(USER_AGENT, HeaderValue::from_static("salah-cli"));
            let coords: Vec<APICoord> = api::fetch::<Vec<APICoord>>(url.as_str(), headers)
                .await
                .with_context(|| {
                    format!(
                        "Could not get coordinates with city = `{}` and country = `{}`",
                        city, country
                    )
                })?;

            if coords.len() < 1 {
                return Err(anyhow::anyhow!("Could not find lat, lng from city = `{}` and country = `{}`. Please check spelling!", city, country));
            }

            let lat = coords[0]
                .lat
                .parse::<f64>()
                .with_context(|| format!("Could not convert `lat` = `{}` to f64", coords[0].lat))?;
            let lng = coords[0]
                .lon
                .parse::<f64>()
                .with_context(|| format!("Could not convert `lng` = `{}` to f64", coords[0].lon))?;

            return Ok(ParsedOptions::Calculation {
                date,
                timezone,
                lat,
                lng,
                timings,
                auth,
                school,
                format,
            });
        }
        Commands::Coord { common, lat, lng } => {
            let date = common
                .parsed_date()
                .with_context(|| format!("Failed to create date with `{}`", common.date))?;
            let timezone = common
                .parsed_timezone()
                .with_context(|| format!("Failed to create timezone with `{}`", common.timezone))?;
            let timings = common
                .parsed_timings()
                .with_context(|| format!("Failed to parse timings with {:?}", common.timings))?;
            let auth = common
                .parsed_auth()
                .with_context(|| format!("Failed to parse authority with `{}`", common.auth))?;
            let school = if common.hanafi {
                types::School::Hanafi
            } else {
                types::School::Shafi
            };
            let format = common.format.to_owned();
            return Ok(ParsedOptions::Calculation {
                date,
                timezone,
                lat: *lat,
                lng: *lng,
                timings,
                auth,
                school,
                format,
            });
        }
        Commands::Timings => {
            return Ok(ParsedOptions::Timings);
        }
        Commands::Authority => {
            return Ok(ParsedOptions::Authority);
        }
    }
}

pub fn stdout_writer() -> std::io::BufWriter<std::io::StdoutLock<'static>> {
    let stdout = std::io::stdout();
    let writer = std::io::BufWriter::new(stdout.lock());
    return writer;
}

pub fn display_timings() {
    let mut writer = stdout_writer();

    writer
        .write(
            format!(
                "{}: {}",
                "Usage".underline(),
                "salah <location | coords> [OPTIONS] [TIMINGS]..."
            )
            .as_bytes(),
        )
        .unwrap();
    writer.write(b"\n").unwrap();
    writer
        .write(b"\nThe below can be passed to [TIMINGS]...")
        .unwrap();
    writer.write(b"\n").unwrap();
    writer
        .write(format!("\n{}:", "Timings".underline()).as_bytes())
        .unwrap();

    for time in types::Timing::list() {
        writer
            .write(
                format!(
                    "\n  {:<width$}{:<width$}",
                    time.to_str(),
                    time.desc(),
                    width = 10
                )
                .as_bytes(),
            )
            .unwrap();
    }

    writer.write(b"\n").unwrap();
    writer.flush().unwrap();
}

pub fn display_authority() {
    let mut writer = stdout_writer();

    writer
        .write(format!("{}: {}", "Usage".underline(), "--auth <AUTH>").as_bytes())
        .unwrap();
    writer.write(b"\n").unwrap();
    writer
        .write(format!("\n{}:", "Explanation".underline()).as_bytes())
        .unwrap();

    writer
        .write(b"\nCalculation authorities are used for the calculation of Fajr and Isha.")
        .unwrap();
    writer.write(b"\nThe time for Fajr is described as dawn; when there is fine white line at the horizon.").unwrap();
    writer.write(b"\nIsha time is described as when the night sky has lost all the light from the sunset.").unwrap();
    writer
        .write(
            b"\nAs this is quite ambiguous, the scholars have differed upon the angle that the sun",
        )
        .unwrap();
    writer
        .write(
            b"\nmakes when these two times occur. Each authority has slightly different angles for",
        )
        .unwrap();
    writer
        .write(b"\nFajr and Isha. Makkah uses a time difference from Maghrib (sunset).")
        .unwrap();
    writer
        .write(
            b"\n\nThe below can be used with the --auth <AUTH> option when calculating timings.\n",
        )
        .unwrap();
    writer
        .write(format!("\n{}:", "Authorities".underline()).as_bytes())
        .unwrap();

    for auth in types::Authority::list() {
        writer
            .write(
                format!(
                    "\n  {:<width$}{:<width$}",
                    auth.to_str(),
                    format!("{} - {}", auth.desc(), auth.name()),
                    width = 10
                )
                .as_bytes(),
            )
            .unwrap();
    }
    writer.write(b"\n").unwrap();

    writer.flush().unwrap();
}
