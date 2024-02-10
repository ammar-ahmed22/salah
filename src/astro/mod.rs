use anyhow::Result;
use chrono::{ DateTime, TimeZone, Datelike, NaiveDate };
use crate::math::*;

/// Returns the Julian Date for the given date
/// 
/// ### Arguments
/// 
/// * `datetime` - A chrono DateTime object
pub fn julian(date: NaiveDate) -> f64 {
  let year: i32 = date.year();
  let month: u32 = date.month();
  let day: u32 = date.day();

  let mut y = year as f64;
  let mut m = month as f64;
  let d = day as f64;
  if m == 1.0 || m == 2.0 {
    y = y - 1.0;
    m = m + 12.0;
  }

  let a = (y / 100.0).floor();
  let b = 2.0 - a + (a / 4.0).floor();
  return (365.25 * (y + 4716.0)).floor() + (30.6001 * (m + 1.0)).floor() + d + b - 1524.5; 
}

/// Returns the Equation of Time and Declination of the Sun for a given Julian Date
/// as per the approximation found at: https://web.archive.org/web/20181115153648/http://aa.usno.navy.mil/faq/docs/SunApprox.php
/// 
/// Equation of Time is in hours (0 - 24)
/// Declination of the Sun is in degrees
/// 
/// ### Arguments
/// 
/// * `jd` - A float value representing the Julian Date
pub fn sun_coords(jd: f64) -> (f64, f64) {
  // All values are in degrees
  // Number of days since Julian Date epoch (2000 Janaury 1.5)
  let j_2000 = jd - 2_451_545.0;

  // Mean anomaly of the sun
  let g = deg::normalize_angle(357.529 + (0.98560028 * j_2000));

  // Mean longitude of the Sun
  let q = deg::normalize_angle(280.459 + (0.98564736 * j_2000));

  // Geocentric apparent ecliptic longitude of the Sun (adjusted for aberration)
  let l = deg::normalize_angle(q + (1.915 * deg::sin(g)) + (0.020 * deg::sin(2.0 * g)));

  // Distance of the Sun from the Earth
  // let r = 1.00014 - (0.01671 * deg::cos(g)) - (0.00014 * deg::cos(2.0 * g));


  // Mean obliquity of the ecliptic, in degrees
  let e = 23.439 - (0.00000036 * j_2000);

  // Right ascension of the Sun
  let mut ra = deg::atan2(deg::cos(e) * deg::sin(l), deg::cos(l)) / 15_f64;
  // Converting to hours
  ra = time::normalize_hour(ra);

  let eqt = (q / 15.0) - ra;

  let decl = deg::asin(deg::sin(e) * deg::sin(l));

  return (eqt, decl);
}

/// Gets the zenith time in hours of the day (0 - 24)
/// 
/// ### Arguments
/// * `jd` - The Julian date
/// * `lng` - The longitude value
/// * `tz` - The timezone offset value
pub fn zenith(jd: f64, lng: f64, tz: f64) -> f64 {
  let eqt = sun_coords(jd).0;
  return 12_f64 + tz - (lng / 15_f64) - eqt;
}

pub enum HorizonDirection {
  Sunrise,
  Sunset
}

/// Gets the hour at which the sun makes a specified angle from the horizon
/// 
/// ### Arguments
/// * `angle` - The angle to calculate the time for (0 would be sunrise/sunset time)
/// * `jd` - The Julian date
/// * `zenith` - The hour time for when the sun hits the zenith
/// * `lat` - The latitude value
/// * `direction` - The direction to calculate the angle for (from Sunrise, from Sunset)
pub fn horizon_hour_angle(angle: f64, jd: f64, zenith: f64, lat: f64, direction: HorizonDirection) -> f64 {
  let decl = sun_coords(jd).1;
  let t_a = (1_f64 / 15_f64) * deg::acos(
    (-deg::sin(angle) - deg::sin(lat)*deg::sin(decl)) /
    (deg::cos(lat)*deg::cos(decl))
  );
  match direction {
    HorizonDirection::Sunrise => {
      return zenith - t_a
    },
    HorizonDirection::Sunset => {
      return zenith + t_a
    }
  }
}

/// Gets the hour at which the shadow is a specified length of a given object
/// e.g. length = 1 -> the shadow is the same size as the object
/// 
/// ### Arguments
/// * `length` - The relative shadow length
/// * `jd` - The Julian date
/// * `zenith` - The hour time for when the sun hits the zenith
/// * `lat` - The latitude value
pub fn shadow_length_hour(length: f64, jd: f64, zenith: f64, lat: f64) -> f64 {
  let decl = sun_coords(jd).1;
  let a_t = (1_f64 / 15_f64) * deg::acos(
    (deg::sin(deg::acot(length + deg::tan(lat - decl))) - (deg::sin(lat) * deg::sin(decl))) /
    (deg::cos(lat) * deg::cos(decl))
  );
  return zenith + a_t;
}