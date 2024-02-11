#[derive(Debug)]
pub enum School {
    Hanafi,
    Shafi,
}

impl School {
    pub fn shadow_length(&self) -> f64 {
        match self {
            Self::Hanafi => 2_f64,
            Self::Shafi => 1_f64,
        }
    }
}

#[derive(Debug)]
pub enum Authority {
    MWL,
    ISNA,
    Egypt,
    Makkah,
    Karachi,
    Tehran,
    Jafari,
}

#[derive(Debug)]
pub enum IshaParam {
    Angle(f64),
    Duration(std::time::Duration),
}

impl Authority {
    pub fn from_str(name: &str) -> Option<Self> {
        let lowercase = name.to_lowercase();
        match lowercase.as_str() {
            "mwl" => Some(Self::MWL),
            "isna" => Some(Self::ISNA),
            "egypt" => Some(Self::Egypt),
            "makkah" => Some(Self::Makkah),
            "karachi" => Some(Self::Karachi),
            "tehran" => Some(Self::Tehran),
            "jafari" => Some(Self::Jafari),
            _ => None,
        }
    }
    pub fn fajr_angle(&self) -> f64 {
        match self {
            Self::MWL => 18_f64,
            Self::ISNA => 15_f64,
            Self::Egypt => 19.5_f64,
            Self::Makkah => 18.5_f64,
            Self::Karachi => 18_f64,
            Self::Tehran => 17.7_f64,
            Self::Jafari => 16_f64,
        }
    }

    pub fn isha_param(&self) -> IshaParam {
        match self {
            Self::MWL => IshaParam::Angle(17_f64),
            Self::ISNA => IshaParam::Angle(15_f64),
            Self::Egypt => IshaParam::Angle(17.5_f64),
            Self::Makkah => IshaParam::Duration(std::time::Duration::from_secs(90 * 3600)),
            Self::Karachi => IshaParam::Angle(18_f64),
            Self::Tehran => IshaParam::Angle(14_f64),
            Self::Jafari => IshaParam::Angle(14_f64),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::MWL => "Muslim World League",
            Self::ISNA => "Islamic Society of North America",
            Self::Egypt => "Egyptian General Authority of Survey",
            Self::Makkah => "Umm al-Qura University, Makkah",
            Self::Karachi => "University of Islamic Sciences, Karachi",
            Self::Tehran => "Institute of Geophysics, University of Tehran",
            Self::Jafari => "Shia Ithna Ashari, Leva Research Institute, Qum",
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::MWL => "MWL",
            Self::ISNA => "ISNA",
            Self::Egypt => "Egypt",
            Self::Makkah => "Makkah",
            Self::Karachi => "Karachi",
            Self::Tehran => "Tehran",
            Self::Jafari => "Jafari",
        }
    }

    pub fn desc(&self) -> &str {
        match self {
            Self::MWL => "Fajr at 18 degrees, Isha at 17 degrees.",
            Self::ISNA => "Fajr at 15 degrees, Isha at 18 degrees.",
            Self::Egypt => "Fajr at 19.5 degrees, Isha at 17.5 degrees",
            Self::Makkah => "Fajr at 18.5 degrees, Isha 90 min after Maghrib.",
            Self::Karachi => "Fajr at 18 degrees, Isha at 18 degrees.",
            Self::Tehran => "Fajr at 17.7 degrees, Isha at 14 degrees.",
            Self::Jafari => "Fajr at 16 degrees, Isha at 14 degrees.",
        }
    }

    pub fn list() -> [Self; 7] {
        return [
            Authority::MWL,
            Authority::ISNA,
            Authority::Egypt,
            Authority::Makkah,
            Authority::Karachi,
            Authority::Tehran,
            Authority::Jafari,
        ];
    }
}

#[derive(Debug)]
pub enum Timing {
    Fajr,
    Sunrise,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
    Midnight,
}

impl Timing {
    pub fn from_str(name: &str) -> Option<Self> {
        let lowercase = name.to_lowercase();
        match lowercase.as_str() {
            "fajr" => Some(Self::Fajr),
            "sunrise" => Some(Self::Sunrise),
            "dhuhr" => Some(Self::Dhuhr),
            "asr" => Some(Self::Asr),
            "maghrib" => Some(Self::Maghrib),
            "isha" => Some(Self::Isha),
            "midnight" => Some(Self::Midnight),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Fajr => "fajr",
            Self::Sunrise => "sunrise",
            Self::Dhuhr => "dhuhr",
            Self::Asr => "asr",
            Self::Maghrib => "maghrib",
            Self::Isha => "isha",
            Self::Midnight => "midnight",
        }
    }

    pub fn desc(&self) -> &str {
        match self {
      Self::Fajr => "The dawn prayer time. Dependent on angle determined by authority (see salah authority)",
      Self::Sunrise => "Sunrise time. Fajr time ends at sunrise.",
      Self::Dhuhr => "The mid-day prayer time.",
      Self::Asr => "The evening prayer time. Dependent on school of thought (Hanafi vs Others).",
      Self::Maghrib => "The sunset prayer time.",
      Self::Isha => "The night prayer time. Dependent on angle determined by authority (see salah authority)",
      Self::Midnight => "The Islamic midnight time. Isha time ends at midnight."
    }
    }

    pub fn list() -> [Self; 7] {
        return [
            Timing::Fajr,
            Timing::Sunrise,
            Timing::Dhuhr,
            Timing::Asr,
            Timing::Maghrib,
            Timing::Isha,
            Timing::Midnight,
        ];
    }
}
