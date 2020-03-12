use serde::{Serialize, Deserialize};
use crate::models::{Weather, Icon};
use chrono::{NaiveDateTime, Datelike, Timelike};

#[derive(Debug, Serialize, Deserialize)]
pub struct DarkSkyReading {
    currently: DarkSkyWeather,
    hourly: DarkSkyPrediction,
}

impl DarkSkyReading {
    pub fn get_weather(self) -> (DarkSkyWeather, Vec<DarkSkyWeather>) {
        return (self.currently, self.hourly.data.into_iter().skip(1).collect());
    }
}

/// Data will generally contain 49 entries
/// The first should be ignored as for the current hour and so should be the exact same as `currently`
#[derive(Debug, Serialize, Deserialize)]
struct DarkSkyPrediction {
    data: Vec<DarkSkyWeather>
}

/// Dark Sky Weather Reading (from `currently` or `hourly`)
/// Units are expected to be SI
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DarkSkyWeather {
    /// Unix timestamp
    time: u64,
    /// One of
    /// clear-day, clear-night, sleet, hail, cloudy, partly-cloudy-day, partly-cloudy-night, snow, fog, thunderstorm, rain
    /// Possibly others too
    icon: String,
    /// 0+
    precip_intensity: f64,
    /// Between 0 and 1, percentage probability of precipitation occurring
    precip_probability: f64,
    /// Air temperature
    temperature: f64,
    /// 'Feels like' temperature
    apparent_temperature: f64,
    /// Between 0 and 1, percentage of sky occluded by clouds
    cloud_cover: f64,
    /// 0+, average wind speed
    wind_speed: f64,
    /// 0+, wind gust speed
    wind_gust: f64,
    /// Between 0 and 1, relative humidity percentage
    humidity: f64,
    /// Optional, one of
    /// `rain`, `snow`, `sleet`
    precip_type: Option<String>,
}

impl Weather {
    pub fn update_id(mut self, origin: &Weather) -> Weather {
        self.id = format!("{}-{:0>3}-{:0>2}-{}-{:0>3}-{:0>2}", self.year, self.day, self.hour, origin.year, origin.day, origin.hour);
        return self;
    }
}

impl From<DarkSkyWeather> for Weather {
    fn from(weather: DarkSkyWeather) -> Self {
        let datetime = NaiveDateTime::from_timestamp(weather.time as i64, 0);
        let year = datetime.year() as u16;
        let day = datetime.ordinal() as u16;
        let hour = datetime.hour() as u8;
        let id = format!("{}-{:0>3}-{:0>2}", year, day, hour);
        return Weather::new(
            id,
            year,
            day,
            hour,
            Icon::from_str(&weather.icon),
            weather.precip_intensity,
            weather.precip_probability,
            weather.apparent_temperature,
            weather.wind_speed,
            weather.wind_gust,
            weather.humidity,
            weather.precip_type,
        );
    }
}

impl From<&DarkSkyWeather> for Weather {
    fn from(weather: &DarkSkyWeather) -> Self {
        let datetime = NaiveDateTime::from_timestamp(weather.time as i64, 0);
        let year = datetime.year() as u16;
        let day = datetime.ordinal() as u16;
        let hour = datetime.hour() as u8;
        let id = format!("{}-{}-{}", year, day, hour);
        return Weather::new(
            id,
            year,
            day,
            hour,
            Icon::from_str(&weather.icon),
            weather.precip_intensity,
            weather.precip_probability,
            weather.apparent_temperature,
            weather.wind_speed,
            weather.wind_gust,
            weather.humidity,
            weather.precip_type.clone(),
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test that the DarkSky* models field names and types match the json
    #[test]
    fn test_dark_sky_parsing() {
        let json_file_path = format!("{}/resources/test/dark_sky_weather.json", env!("CARGO_MANIFEST_DIR"));
        let weather_json = std::fs::read_to_string(json_file_path).unwrap();
        let weather: DarkSkyReading = serde_json::from_str(&weather_json).unwrap();

        assert_eq!(weather.currently.time, 1574395263);
        assert_eq!(weather.currently.icon, "partly-cloudy-night");
        assert_eq!(weather.currently.precip_intensity, 0.);
        assert_eq!(weather.currently.precip_probability, 0.);
        assert_eq!(weather.currently.temperature, 6.56);
        assert_eq!(weather.currently.apparent_temperature, 3.69);
        assert_eq!(weather.currently.cloud_cover, 0.66);
        assert_eq!(weather.currently.wind_speed, 4.18);
        assert_eq!(weather.currently.wind_gust, 8.6);
        assert_eq!(weather.currently.humidity, 0.89);
        assert!(weather.currently.precip_type.is_none());

        assert_eq!(weather.hourly.data.len(), 49);


        assert_eq!(weather.hourly.data[25].time, 1574485200);
        assert_eq!(weather.hourly.data[25].icon, "rain");
        assert_eq!(weather.hourly.data[25].precip_intensity, 0.7018);
        assert_eq!(weather.hourly.data[25].precip_probability, 0.58);
        assert_eq!(weather.hourly.data[25].temperature, 8.44);
        assert_eq!(weather.hourly.data[25].apparent_temperature, 4.91);
        assert_eq!(weather.hourly.data[25].cloud_cover, 1.);
        assert_eq!(weather.hourly.data[25].wind_speed, 7.05);
        assert_eq!(weather.hourly.data[25].wind_gust, 13.01);
        assert_eq!(weather.hourly.data[25].humidity, 0.85);
        assert_eq!(weather.hourly.data[25].precip_type.as_ref().unwrap(), "rain");
    }

    #[test]
    fn test_into_weather() {
        let json_file_path = format!("{}/resources/test/dark_sky_weather.json", env!("CARGO_MANIFEST_DIR"));
        let weather_json = std::fs::read_to_string(json_file_path).unwrap();
        let weather_reading: DarkSkyReading = serde_json::from_str(&weather_json).unwrap();

        let _current_weather: Weather = weather_reading.currently.into();
        let _rainy_weather: Weather = (&weather_reading.hourly.data[24]).into();
    }
}
