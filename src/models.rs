use serde::{Serialize, Deserialize};
use log::error;
use rusqlite::types::{ToSql, FromSql, FromSqlResult, ValueRef};
use rusqlite::Error;
use rusqlite::types::ToSqlOutput;
use std::fmt::Display;
use serde::export::Formatter;
use std::fmt;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    /// ID of weather reading, should be '{year}-{day}-{hour}'
    pub id: String,
    /// Unix timestamp of reading
    pub timestamp: i64,
    /// UTC year of reading
    pub year: u16,
    /// UTC day of year of reading
    pub day: u16,
    /// UTC 24 hour of reading (0 - 23)
    pub hour: u8,
    /// Icon for weather
    pub icon: Icon,
    /// Precipitation in millimeters per hour
    pub precip_intensity: f64,
    /// Percentage probability of precipitation occurring
    pub precip_probability: f64,
    /// 'Feels like' temperature in celsius
    pub temp: f64,
    /// Average wind speed in meters per hour
    pub wind_speed: f64,
    /// Wind gust speed in meters per hour
    pub wind_gust: f64,
    /// Relative humidity percentage
    pub humidity: f64,
    // Optional type of precipitation (only `rain`, `snow`, `sleet` and `None` are supported)
    pub precip_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prediction {
    /// ID of weather reading, should be '{prediction_year}-{prediction_day}-{prediction_hour}-{reading_year}-{reading_day}-{reading_hour}'
    pub id: String,
    /// UTC year of reading
    pub reading_year: u16,
    /// UTC day of year of reading
    pub reading_day: u16,
    /// UTC 24 hour of reading (0 - 23)
    pub reading_hour: u8,
    /// UTC year being predicted
    pub prediction_year: u16,
    /// UTC day of year being predicted
    pub prediction_day: u16,
    /// UTC 24 hour being predicted (0 - 23)
    pub prediction_hour: u8,
    /// How many hours forward is the prediction from the reading
    pub hour_diff: u8,
    /// Icon for weather
    pub icon: Icon,
    /// Precipitation in millimeters per hour
    pub precip_intensity: f64,
    /// Percentage probability of precipitation occurring
    pub precip_probability: f64,
    /// 'Feels like' temperature in celsius
    pub temp: f64,
    /// Average wind speed in meters per hour
    pub wind_speed: f64,
    /// Wind gust speed in meters per hour
    pub wind_gust: f64,
    /// Relative humidity percentage
    pub humidity: f64,
    // Optional type of precipitation (only `rain`, `snow`, `sleet` and `None` are supported)
    pub precip_type: Option<String>,
}

impl Weather {
    pub fn new(id: String, year: u16, day: u16, hour: u8, icon: Icon, precip_intensity: f64, precip_probability: f64, temp: f64, wind_speed: f64, wind_gust: f64, humidity: f64, precip_type: Option<String>) -> Weather {
        let timestamp = Into::<NaiveDateTime>::into(SimpleDate::new(year, day, hour)).timestamp();
        return Weather { id, timestamp, year, day, hour, icon, precip_intensity, precip_probability, temp, wind_speed, wind_gust, humidity, precip_type };
    }
}

impl Prediction {
    pub fn new(id: String, reading_year: u16, reading_day: u16, reading_hour: u8, prediction_year: u16, prediction_day: u16, prediction_hour: u8, hour_diff: u8, icon: Icon, precip_intensity: f64, precip_probability: f64, temp: f64, wind_speed: f64, wind_gust: f64, humidity: f64, precip_type: Option<String>) -> Prediction {
        return Prediction { id, reading_year, reading_day, reading_hour, prediction_year, prediction_day, prediction_hour, hour_diff, icon, precip_intensity, precip_probability, temp, wind_speed, wind_gust, humidity, precip_type };
    }
}

impl Weather {
    pub fn simple_date(&self) -> SimpleDate {
        SimpleDate::new(self.year, self.day, self.hour)
    }

    pub fn date(&self) -> NaiveDateTime {
        self.simple_date().into()
    }
}

impl Prediction {
    pub fn simple_reading_date(&self) -> SimpleDate {
        SimpleDate::new(self.reading_year, self.reading_day, self.reading_hour)
    }

    pub fn simple_prediction_date(&self) -> SimpleDate {
        SimpleDate::new(self.prediction_year, self.prediction_day, self.prediction_hour)
    }

    pub fn reading_date(&self) -> NaiveDateTime {
        self.simple_reading_date().into()
    }

    pub fn prediction_date(&self) -> NaiveDateTime {
        self.simple_prediction_date().into()
    }
}

impl Display for Weather {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r#"
ID        {}
Year      {}
Day       {}
Hour      {}
Icon      {}
P. Amt    {} mm/h
P. Prob   {}%
P. Type   {}
Temp      {}°C
W. Speed  {} m/s
W. Gust   {} m/s
Humidity  {}%
        "#,
               self.id,
               self.year,
               self.day,
               self.hour,
               self.icon.to_str(),
               self.precip_intensity,
               self.precip_probability * 100.,
               self.precip_type.as_ref().unwrap_or(&String::from("None")),
               self.temp,
               self.wind_speed,
               self.wind_gust,
               self.humidity * 100.)
    }
}

impl Display for Prediction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r#"
 ID        {}
 R Year    {}
 R Day     {}
 R Hour    {}
 P Year    {}
 P Day     {}
 P Hour    {}
 Hour diff {}
 Icon      {}
 P. Amt    {} mm/h
 P. Prob   {}%
 P. Type   {}
 Temp      {}°C
 W. Speed  {} m/s
 W. Gust   {} m/s
 Humidity  {}%
        "#,
               self.id,
               self.reading_year,
               self.reading_day,
               self.reading_hour,
               self.prediction_year,
               self.prediction_day,
               self.prediction_hour,
               self.hour_diff,
               self.icon.to_str(),
               self.precip_intensity,
               self.precip_probability * 100.,
               self.precip_type.as_ref().unwrap_or(&String::from("None")),
               self.temp,
               self.wind_speed,
               self.wind_gust,
               self.humidity * 100.)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Icon {
    Rain,
    Thunderstorm,
    Fog,
    Snow,
    PartlyCloudy,
    Cloudy,
    Hail,
    Sleet,
    Clear,
    Wind,

    Unknown,
}

impl Icon {
    /// Attempt to convert a string into an icon
    ///
    /// Will return `Icon::Unknown` and log error for any unrecognised input
    pub fn from_str(name: &str) -> Icon {
        return match name {
            "rain" => Icon::Rain,
            "cloudy" => Icon::Cloudy,
            "clear-day" | "clear-night" => Icon::Clear,
            "fog" => Icon::Fog,
            "hail" => Icon::Hail,
            "thunderstorm" => Icon::Thunderstorm,
            "snow" => Icon::Snow,
            "sleet" => Icon::Sleet,
            "wind" => Icon::Wind,
            "partly-cloudy-day" | "partly-cloudy-night" => Icon::PartlyCloudy,
            _ => {
                error!("Unknown icon: {}", name);
                Icon::Unknown
            }
        };
    }

    pub fn to_str(&self) -> &'static str {
        return match self {
            Icon::Rain => "rain",
            Icon::Thunderstorm => "thunderstorm",
            Icon::Fog => "fog",
            Icon::Snow => "snow",
            Icon::PartlyCloudy => "partly-cloudy-day",
            Icon::Cloudy => "cloudy",
            Icon::Hail => "hail",
            Icon::Sleet => "sleet",
            Icon::Clear => "clear-day",
            Icon::Wind => "wind",
            Icon::Unknown => "unknown"
        };
    }
}

impl ToSql for Icon {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, Error> {
        Ok(ToSqlOutput::from(self.to_str()))
    }
}

impl FromSql for Icon {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::from(value.as_str().map(|str| Icon::from_str(str)))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherReading {
    current: Weather,
    prediction: Vec<Weather>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SimpleDate {
    /// Four digit year (e.g. 2020)
    pub year: u16,
    /// Day of year, one based (i.e. first day is 1)
    pub day: u16,
    /// Hour of day, zero based, 24 hour (i.e. first hour is 0 and last is 23)
    pub hour: u8
}

impl SimpleDate {
    /// # Params
    /// Year: Four digit year (e.g. 2020)
    /// Day: Day of year, one based (i.e. first day is 1)
    /// Hour: our of day, zero based, 24 hour (i.e. first hour is 0 and last is 23)
    pub fn new(year: u16, day: u16, hour: u8) -> SimpleDate {
        return SimpleDate {
            year,
            day,
            hour
        };
    }
}

impl From<SimpleDate> for (u16, u16, u8) {
    fn from(value: SimpleDate) -> Self {
        return (value.year, value.day, value.hour);
    }
}