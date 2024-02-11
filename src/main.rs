use anyhow::{Context, Result};
// use chrono::{ NaiveDate, NaiveDateTime, TimeZone, Utc };
// use colored::*;
// use chrono_tz::Tz; 
// use salah::times::PrayerTimes;
// use clap::Parser;
use salah::cli;

/// USEFUL LINKS:
/// https://data.iana.org/time-zones/tzdb-2024a/zone1970.tab -> timezone names
/// https://nominatim.openstreetmap.org/search?city=Whitby&country=Canada&format=json -> lat,lng API 

fn main() -> Result<()> {
    // CONSTANTS
    // const TIMEZONE_NAME: &str = "America/Toronto";
    // const LAT: f64 = 43.87982_f64;
    // const LNG: f64 = -78.9421751_f64;

    // Creating DateTime
    let opts = cli::parse()
        .with_context(|| "Failed to parse CLI arguments")?;

    match opts {
        cli::ParsedOptions::Calculation { date, timezone, lat, lng, timings, auth, school } => {
            println!("date = {:?}", date);
            println!("timezone = {:?}", timezone);
            println!("lat = {:?}, lng = {:?}", lat, lng);
            println!("timings = {:?}", timings);
            println!("auth = {:?}", auth);
            println!("school = {:?}", school);
        },
        cli::ParsedOptions::Timings => cli::display_timings(),
        cli::ParsedOptions::Authority => cli::display_authority()
    }
    

    return Ok(());
}
