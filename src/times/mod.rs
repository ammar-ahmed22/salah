use anyhow::{ Result};
use chrono::NaiveDateTime;
use chrono::{  NaiveDate, NaiveTime, TimeZone, Utc };
use chrono_tz::Tz;
use crate::astro;
use crate::datetime;
use crate::math;

pub enum School {
  Hanafi,
  Shafi
}

impl School {
  fn shadow_length(&self) -> f64 {
    match self {
      Self::Hanafi => 2_f64,
      Self::Shafi => 1_f64
    }
  }
}

pub enum Authority {
  MWL,
  ISNA,
  EGAS,
  UQUM,
  UISK,
  IGUT,
  SIA
}

pub enum IshaParam {
  angle(f64),
  duration(std::time::Duration)
}

impl Authority {
  fn fajr_angle(&self) -> f64 {
    match self {
      Self::MWL => 18_f64,
      Self::ISNA => 15_f64,
      Self::EGAS => 19.5_f64,
      Self::UQUM => 18.5_f64,
      Self::UISK => 18_f64,
      Self::IGUT => 17.7_f64,
      Self::SIA => 16_f64
    }
  }

  fn isha_param(&self) -> IshaParam {
    match self {
      Self::MWL => IshaParam::angle(17_f64),
      Self::ISNA => IshaParam::angle(15_f64),
      Self::EGAS => IshaParam::angle(17.5_f64),
      Self::UQUM => IshaParam::duration(std::time::Duration::from_secs(90 * 3600)),
      Self::UISK => IshaParam::angle(18_f64),
      Self::IGUT => IshaParam::angle(14_f64),
      Self::SIA => IshaParam::angle(14_f64)
    }
  }
}

pub struct PrayerTimes {
  /// timezone
  tz: Tz,

  /// Latitude
  lat: f64,

  /// Longitude
  lng: f64,

  // Date
  date: NaiveDate,

  // Timezone offset from GMT
  tz_offset: f64,
  
  // Julian date
  jd: f64,

  // Calculation authority
  auth: Authority,

  // School of thought for jurisprudence
  school: School,
}

impl PrayerTimes {
  // =================== Constructors ============================
  /// Creates a new prayer time struct with default parameters
  /// 
  /// ### Arguments
  /// * `lat` - The latitude value to calculate for
  /// * `lng` - The longitude value to calculate for
  /// 
  /// ### Defaults
  /// * `tz` - Timezone default is set to `America/Toronto`
  /// * `date` - Date defaults to today's date
  /// * `auth` - Calculation authority defaults to Islamic Society of North America (ISNA) (used for fajr and isha time)
  /// * `school` - School of thought for jurisprudence (used for asr timing) defaults to Hanafi
  pub fn new(lat: f64, lng: f64) -> PrayerTimes {
    const DEFAULT_TZ: &str = "America/Toronto";
    let tz: Tz = DEFAULT_TZ.parse().expect("Invalid time zone!");
    let default_date = tz.from_utc_datetime(&Utc::now().naive_utc()).date_naive();

    return PrayerTimes { 
      tz, 
      lat, 
      lng, 
      date: default_date, 
      tz_offset: datetime::tz_offset(tz), 
      jd: astro::julian(default_date),
      auth: Authority::ISNA,
      school: School::Hanafi
    };
  }

  // =============== Setters =================
  /// Sets the date to compute timings for
  pub fn with_date(mut self, date: &NaiveDate) -> Self {
    self.date = *date;
    self.jd = astro::julian(*date);
    return self;
  }

  /// Sets the timezone
  pub fn with_timezone(mut self, tz: Tz) -> Self {
    self.tz = tz;
    self.tz_offset = datetime::tz_offset(tz);
    return self;
  }

  /// Sets the calculation authority
  pub fn with_authority(mut self, auth: Authority) -> Self {
    self.auth = auth;
    return self;
  }

  /// Sets the school of thought
  pub fn with_school(mut self, school: School) -> Self {
    self.school = school;
    return self;
  }

  // ================= Private Methods =======================
  fn zenith(&self) -> f64 {
    return astro::zenith(self.jd, self.lng, self.tz_offset);
  }

  // ================= Public Methods ========================
  /// Returns the fajr (dusk) prayer time
  pub fn fajr(&self) -> NaiveTime {
    let angle = self.auth.fajr_angle();
    let hour = astro::horizon_hour_angle(angle, self.jd, self.zenith(), self.lat, astro::HorizonDirection::Sunrise);
    return datetime::hour2time(hour, true).expect("RangeError @ PrayerTime.fajr");
  }

  /// Returns the dhuhr (mid-day) prayer time 
  pub fn dhuhr(&self) -> NaiveTime {
    return datetime::hour2time(self.zenith(), true).expect("RangeError @ PrayerTime.dhuhr");
  }  

  /// Returns the asr (evening) prayer time
  pub fn asr(&self) -> NaiveTime {
    let hour = astro::shadow_length_hour(self.school.shadow_length(), self.jd, self.zenith(), self.lat);
    return datetime::hour2time(hour, true).expect("RangeError @ PrayerTime.asr");
  }

  /// Returns the maghrib (sunset) prayer time
  pub fn maghrib(&self) -> NaiveTime {
    let hour = astro::horizon_hour_angle(0.833, self.jd, self.zenith(), self.lat, astro::HorizonDirection::Sunset);
    return datetime::hour2time(hour, true).expect("RangeError @ PrayerTime.maghrib");
  }

  /// Returns the isha (night) prayer time
  pub fn isha(&self) -> NaiveTime {
    let param = self.auth.isha_param();
    return match param {
      IshaParam::angle(a) => {
        let hour = astro::horizon_hour_angle(a, self.jd, self.zenith(), self.lat, astro::HorizonDirection::Sunset);
        let time = datetime::hour2time(hour, true).expect("RangeError @ PrayerTime.isha");
        time
      },
      IshaParam::duration(d) => {
        let maghrib = datetime::time2hour(self.maghrib());
        let sunset = datetime::hour2time(maghrib, true).expect("RangeError @ PrayerTime.isha");
        sunset + d
      }
    }
  }

  /// Returns the sunrise time
  pub fn sunrise(&self) -> NaiveTime {
    let hour = astro::horizon_hour_angle(0.833, self.jd, self.zenith(), self.lat, astro::HorizonDirection::Sunrise);

    return datetime::hour2time(hour, true).expect("RangeError @ PrayerTime.sunrise");
  }

  /// Returns the midnight time
  pub fn midnight(&self) -> NaiveTime {

    let sunrise = datetime::time2hour(self.sunrise());
    let sunset = datetime::time2hour(self.maghrib());

    let mid = sunset + math::time::normalize_hour(sunrise - sunset) / 2_f64;
    return datetime::hour2time(mid, true).expect("RangeError @ PrayerTime.midnight");
  }

}