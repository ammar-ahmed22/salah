use anyhow::{Context, Result};
use salah::cli;
use salah::times;

/// USEFUL LINKS:
/// https://data.iana.org/time-zones/tzdb-2024a/zone1970.tab -> timezone names
/// https://nominatim.openstreetmap.org/search?city=Whitby&country=Canada&format=json -> lat,lng API

#[tokio::main]
async fn main() -> Result<()> {
    let opts = cli::parse()
        .await
        .with_context(|| "Failed to parse CLI arguments")?;

    match opts {
        cli::ParsedOptions::Calculation {
            date,
            timezone,
            lat,
            lng,
            timings,
            auth,
            school,
            format,
        } => {
            let pt = times::PrayerTimes::new(lat, lng)
                .with_date(&date)
                .with_timezone(&timezone)
                .with_authority(&auth)
                .with_school(&school);
            
            for timing in &timings {
                println!("{} {}", timing.to_str(), pt.timing(timing).format(format.as_str()));
            }
        }
        cli::ParsedOptions::Timings => cli::display_timings(),
        cli::ParsedOptions::Authority => cli::display_authority(),
    }

    return Ok(());
}
