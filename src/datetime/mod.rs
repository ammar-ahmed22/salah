use chrono::{ DateTime, TimeZone, Offset, NaiveTime, Utc, Timelike };
use anyhow::{Context, Result};
use chrono_tz::Tz;

#[cfg(test)]
mod tests {
  use chrono::{ NaiveTime, Timelike };
  use crate::datetime::{ time2hour, hour2time };
  #[test]
  fn test_time2hour() {
    let time = NaiveTime::from_hms_opt(17, 24, 0).expect("Error!");
    let hour = time2hour(time);
    assert_eq!(hour, 17.4_f64);
  }

  #[test]
  fn test_hour2time() {
    let hour = 17.4_f64;
    let time = hour2time(hour, true).expect("Error!");
    assert_eq!(time.hour(), 17);
    assert_eq!(time.minute(), 24);
    assert_eq!(time.second(), 0);
  }
}

/// Returns the Timezone offset from UTC in hours
/// 
/// ### Arguments
/// * `tz` - A `chrono_tz` Tz object
pub fn tz_offset(tz: Tz) -> f64 {
  let dt = tz.from_utc_datetime(&Utc::now().naive_utc());
  return get_tz_offset(dt);
}


fn get_tz_offset<Tz: TimeZone>(datetime: DateTime<Tz>) -> f64 {
  return (datetime.offset().fix().local_minus_utc() as f64) / 3600.0;
}

/// Creates NaiveTime object from hour value
/// 
/// ### Arguments
/// * `hour` - A fractional value representing the hour of the day (0-24)
/// * `round_seconds` - if `true`, minutes will be rounded by the seconds value and seconds will always be zero
pub fn hour2time(hour: f64, round_seconds: bool) -> Result<NaiveTime> {
  let mut h = hour.trunc() as u32;
  let d = (hour - hour.trunc()) * 60.0;
  let mut m = d as u32;
  let mut s = ((d - d.trunc()) * 60.0).round() as u32;

  if h == 24 {
    h = 0;
  }

  if s >= 60 {
    s -= 60;
    m += 1;
  }

  if m >= 60 {
    m -= 60;
  }

  if round_seconds {
    if s >= 30 {
      m += 1;
      
    }
    s = 0;
  }
  let time = match NaiveTime::from_hms_opt(h, m, s) {
    None => Err(anyhow::anyhow!("datetime::hour2time (out of range)")),
    Some(t) => { Ok(t) }
  }
    .with_context(|| format!("Cannot create NaiveTime with hour = `{}`, minute = `{}`, second = `{}`", h, m, s))?;
  
  return Ok(time);
}

/// Creates fractional hour from NaiveTime
/// 
/// ### Arguments
/// * `time` - NaiveTime object
pub fn time2hour(time: NaiveTime) -> f64 {
  let hour = time.hour() as f64;
  let minutes = time.minute() as f64;
  let seconds = time.second() as f64;

  let decimal = (minutes + (seconds / 60_f64)) / 60_f64;

  return hour + decimal;
} 
