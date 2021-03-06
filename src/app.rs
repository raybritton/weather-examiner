use crate::db_manager::DbManager;
use crate::Error;
use std::path::PathBuf;
use std::fs;
use log::{error, trace};
use crate::templates::DarkSkyReading;
use crate::models::{Weather, Prediction, SimpleDate};
use chrono::NaiveDateTime;
use crate::extensions::Utils;

pub struct WeatherApp {
    db_manager: DbManager
}

impl WeatherApp {
    pub fn new(db_manager: DbManager) -> WeatherApp {
        return WeatherApp {
            db_manager
        };
    }
}

impl WeatherApp {
    pub fn get_reading(&mut self, year: u16, day: u16, hour: u8) -> Result<Weather, Error> {
        return self.db_manager.get_specific_reading(year, day, hour)
            .map_err(|err| err.into());
    }

    pub fn get_reading_with_predictions(&mut self, year: u16, day: u16, hour: u8) -> Result<(Weather, Vec<Prediction>), Error> {
        let weather = self.db_manager.get_specific_reading(year, day, hour)
            .map_err(|err| err.into());
        let predictions = self.db_manager.get_predictions_for(year, day, hour)
            .map_err(|err| err.into());

        return if let Ok(weather) = weather {
            if let Ok(predictions) = predictions {
                Ok((weather, predictions))
            } else {
                Err(predictions.unwrap_err())
            }
        } else {
            Err(weather.unwrap_err())
        }
    }

    /// Get hourly reads from start to end (inclusive, inclusive)
    ///
    /// # Errors
    /// Database errors
    /// No readings in database
    ///
    /// # Returns
    /// List of readings
    ///
    pub fn get_readings_over_range(&mut self, start: SimpleDate, end: SimpleDate) -> Result<Vec<Weather>, Error> {
        let start = Into::<NaiveDateTime>::into(start).timestamp();
        let end = Into::<NaiveDateTime>::into(end).timestamp();
        return self.db_manager.get_readings_over_range(start, end)
            .map_err(|err| err.into());
    }

    /// Get the first reading
    ///
    /// # Errors
    /// Database errors
    /// No readings in database
    ///
    /// # Returns
    /// First reading (current, not predication)
    ///
    pub fn get_first_reading(&mut self) -> Result<Weather, Error> {
        return self.db_manager.get_readings("ASC", 1)
            .map(|mut list| list.pop().expect("No readings"))
            .map_err(|err| err.into());
    }

    /// Get the last reading
    ///
    /// # Errors
    /// Database errors
    /// No readings in database
    ///
    /// # Returns
    /// Last reading (current, not predication)
    ///
    pub fn get_last_reading(&mut self) -> Result<Weather, Error> {
        return self.db_manager.get_readings("DESC", 1)
            .map(|mut list| list.pop().expect("No readings"))
            .map_err(|err| err.into())
    }

    /// Gets the first and last record and returns a list of all missing hour slot between them
    ///
    /// # Errors
    /// Database errors
    ///
    /// # Returns
    /// List of DateTime, each one representing a missing slot (minute and second will be 0)
    /// Will be empty if no missing slots
    ///
    pub fn check_for_missing_data(&mut self) -> Result<Vec<NaiveDateTime>, Error> {
        let readings = self.db_manager.get_all_readings()?;

        let mut date_times = readings.into_iter()
            .map(|weather|  weather.date())
            .collect::<Vec<NaiveDateTime>>();

        date_times.sort();

        if date_times.is_empty() {
            return Ok(vec![]);
        } else if date_times.len() < 3 {
            return Ok(date_times);
        }

        let start = date_times.first().expect("No first date time");
        let end = date_times.last().expect("No last date time");
        let mut results = vec![];
        let mut current = start.clone();

        while &current < end {
            if !date_times.contains(&current) {
                results.push(current.clone())
            }
            current = current.plus_one_hour();
        }

        Ok(results)
    }

    /// Import all json files from a directory
    ///
    /// # Errors
    /// Directory is inaccessible
    /// Path is not a directory
    ///
    pub fn import_data(&mut self, dir: String) -> Result<(), Error> {
        let path = PathBuf::from(dir);
        if path.is_dir() {
            let (files, errors) = self.list_files(path)?;
            for error in errors {
                error!("{}", error);
            }
            for file in files {
                if let Err(err) = self.import_data_from_file(&file) {
                    error!("Failed to import from {:?}: {}", file, err);
                }
            }
        } else {
            return Err(Error::from("Not a directory"));
        }

        Ok(())
    }

    /// Import data from specific file
    ///
    /// # Errors
    /// Failed to read file
    /// Failed to parse json
    /// Failed to insert into database
    ///
    fn import_data_from_file(&mut self, file: &PathBuf) -> Result<(), Error> {
        let json = fs::read_to_string(file)?;
        let dark_sky_weather: DarkSkyReading = serde_json::from_str(&json)?;
        let (current, future) = dark_sky_weather.get_weather();
        let current_weather = current.into();
        let future_weathers = future.into_iter().map(|weather| Into::<Weather>::into(weather).update_id(&current_weather)).collect();

        self.db_manager.add_weather(current_weather, future_weathers)?;

        trace!("Imported {}", file.to_string_lossy().into_owned());

        Ok(())
    }

    /// Gets a list of valid json files in `path`.
    ///
    /// # Errors
    /// Directory is inaccessible
    ///
    /// # Returns
    /// List of paths to valid json files and file access errors
    /// If the results are entirely errors then check:
    /// - Permissions of files within the dir
    /// - Filenames (and all parent directories names) are valid UTF-8
    ///
    fn list_files(&mut self, path: PathBuf) -> Result<(Vec<PathBuf>, Vec<String>), Error> {
        let dir = fs::read_dir(path)?;

        let mut files = vec![];
        let mut errors = vec![];

        for file in dir {
            match file {
                Ok(entry) => {
                    files.push(entry.path());
                }
                Err(err) => {
                    errors.push(format!("{:?}: {}", err.kind(), err));
                }
            }
        }

        let mut filtered: Vec<PathBuf> = files.into_iter()
            .filter(|entry| entry.is_file())
            .filter(|entry| entry.file_name().is_some())
            .filter(|entry| entry.file_name().unwrap().to_str().is_some())
            .filter(|entry| entry.file_name().unwrap().to_str().unwrap().ends_with(".json"))
            .collect();

        filtered.sort();

        return Ok((filtered, errors));
    }
}
