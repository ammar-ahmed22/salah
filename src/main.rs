use anyhow::{ Context, Result };
use chrono::{ NaiveDate, NaiveDateTime, TimeZone, Utc };
use chrono_tz::Tz; 
use salah::times::PrayerTimes;

/// USEFUL LINKS:
/// https://data.iana.org/time-zones/tzdb-2024a/zone1970.tab -> timezone names

fn main() -> Result<()> {
    // CONSTANTS
    const TIMEZONE_NAME: &str = "America/Toronto";
    const LAT: f64 = 43.87982_f64;
    const LNG: f64 = -78.9421751_f64;

    // Creating DateTime
    let utc = Utc::now();
    let naive_dt = NaiveDateTime::new(utc.date_naive(), utc.time());
    let timezone: Tz = match TIMEZONE_NAME.parse() {
        Ok(tz) => tz,
        Err(msg) => { return Err(anyhow::anyhow!(msg))}
    };
    
    let dt = timezone.from_utc_datetime(&naive_dt);
    let jd: f64 = salah::astro::julian(dt.date_naive());

    println!("Today's Julian Date: {}", jd);

    let pt = PrayerTimes::new(LAT, LNG)
        .with_date(&NaiveDate::from_ymd_opt(2024, 2, 9).unwrap())
        .with_timezone(timezone)
        .with_school(salah::times::School::Hanafi)
        .with_authority(salah::times::Authority::ISNA);


    println!("Fajr: {}", pt.fajr());
    println!("Sunrise: {}", pt.sunrise());
    println!("Dhuhr: {}", pt.dhuhr());
    println!("Asr: {}", pt.asr());
    println!("Maghrib: {}", pt.maghrib());
    println!("Isha: {}", pt.isha());
    println!("Midnight: {}", pt.midnight());
    // let _ = pt.midnight();
    return Ok(());
}
