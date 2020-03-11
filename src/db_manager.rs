use rusqlite::{params, Connection, NO_PARAMS, Error, Transaction, Row};
use crate::Error as CrateError;
use log::{trace, debug, error};
use crate::models::{Weather, Icon, Prediction};

const CREATE_WEATHER_TABLE: &str = "CREATE TABLE IF NOT EXISTS weather (id TEXT PRIMARY KEY, year INTEGER, day INTEGER, hour INTEGER, icon TEXT, precip_intensity REAL, precip_probability REAL, temp REAL, wind_speed REAL, wind_gust REAL, humidity REAL, precip_type TEXT)";
const CREATE_WEATHER_UNIQUE_INDEX: &str = "CREATE UNIQUE INDEX IF NOT EXISTS year_day_hour ON weather (year, day, hour)";
const CREATE_PREDICATION_TABLE: &str = "CREATE TABLE IF NOT EXISTS prediction (id TEXT PRIMARY KEY, reading_year INTEGER, reading_day INTEGER, reading_hour INTEGER, prediction_year INTEGER, prediction_day INTEGER, prediction_hour INTEGER, hour_diff INTEGER, icon TEXT, precip_intensity REAL, precip_probability REAL, temp REAL, wind_speed REAL, wind_gust REAL, humidity REAL, precip_type TEXT)";
const CREATE_PREDICATION_UNIQUE_INDEX: &str = "CREATE UNIQUE INDEX IF NOT EXISTS year_day_hour_diff ON prediction (reading_year, reading_day, reading_hour, prediction_year, prediction_day, prediction_hour)";

const DB_VERSION: usize = 1;

pub struct DbManager {
    conn: Connection
}

impl DbManager {
    pub fn new<T: Into<String>>(db_file: T) -> Result<DbManager, CrateError> {
        let path = db_file.into();
        trace!("Using database at {}", path);
        Connection::open(&path)
            .map(|conn| DbManager { conn })
            .map_err(|err| err.into())
    }
}

impl DbManager {
    /// Initialize database
    /// *This must be called before using the database*
    ///
    /// Will migrates database and update user version if needed
    ///
    /// # Panics
    /// If the current database version is unexpected then will exit process (via `process::exit`, not panic)
    ///
    /// # Errors
    /// If any read or write statements fail
    ///
    pub fn init(&mut self) -> Result<(), CrateError> {
        let ver = self.conn.get_user_version()?;
        trace!("Database version starting at {}", ver);
        match ver {
            0 => {
                self.conn.execute(CREATE_WEATHER_TABLE, NO_PARAMS)?;
                self.conn.execute(CREATE_WEATHER_UNIQUE_INDEX, NO_PARAMS)?;
                self.conn.execute(CREATE_PREDICATION_TABLE, NO_PARAMS)?;
                self.conn.execute(CREATE_PREDICATION_UNIQUE_INDEX, NO_PARAMS)?;
                self.conn.set_user_version(DB_VERSION)?;
                debug!("Created weather table, set db version to 1");
            }
            1 => {
                trace!("Database up to date");
            }
            _ => {
                error!("Unknown database version: {}", ver);
                std::process::exit(1);
            }
        }

        Ok(())
    }

    pub fn get_specific_reading(&mut self, year: u16, day: u16, hour: u8) -> Result<Weather, Error> {
        self.conn.query_row("SELECT id, year, day, hour, icon, precip_intensity, precip_probability, temp, wind_speed, wind_gust, humidity, precip_type FROM weather WHERE year = ? AND day = ? AND hour = ?", &[year, day, hour as u16], |row| Ok(DbManager::build_weather(row)))
    }

    pub fn get_predictions_for(&mut self, year: u16, day: u16, hour: u8) -> Result<Vec<Prediction>, Error> {
        let mut statement = self.conn.prepare(&format!("SELECT id, reading_year, reading_day, reading_hour, prediction_year, prediction_day, prediction_hour, hour_diff, icon, precip_intensity, precip_probability, temp, wind_speed, wind_gust, humidity, precip_type FROM prediction WHERE prediction_year = ? AND prediction_day = ? AND prediction_hour = ? ORDER BY hour_diff"))?;
        let predictions = statement.query_map(&[year, day, hour as u16], |row| {
            return Ok(DbManager::build_prediction(row));
        })?
            .map(|prediction| prediction.unwrap())
            .collect();

        return Ok(predictions);
    }

    pub fn get_readings(&mut self, sort: &str, count: usize) -> Result<Vec<Weather>, Error> {
        let mut statement = self.conn.prepare(&format!("SELECT id, year, day, hour, icon, precip_intensity, precip_probability, temp, wind_speed, wind_gust, humidity, precip_type FROM weather ORDER BY id {} LIMIT {}", sort, count))?;
        let weathers = statement.query_map(NO_PARAMS, |row| {
            return Ok(DbManager::build_weather(row));
        })?
            .map(|weather| weather.unwrap())
            .collect();

        return Ok(weathers);
    }

    /// Insert weather reading and it's predictions into the database
    ///
    /// # Errors
    /// Failed to start transaction
    /// Failed to insert data
    /// Failed to commit transaction
    ///
    pub fn add_weather(&mut self, weather: Weather, predictions: Vec<Weather>) -> Result<(), CrateError> {
        let transaction = self.conn.transaction()?;

        DbManager::insert_weather(&transaction, &weather)?;

        predictions.iter()
            .enumerate()
            .try_for_each(|(i, prediction)| {
                DbManager::insert_prediction(&transaction, &weather, &prediction, i + 1) //plus hour because the first one is the next hour (so diff is 1 not 0)
            })?;

        transaction.commit()?;

        Ok(())
    }

    /// Return a list of all readings (current, not predictions)
    ///
    /// # Errors
    /// Failed to read data
    ///
    /// # Returns
    /// List of all readings (sorted by id, which should be oldest to newest)
    ///
    pub fn get_all_readings(&mut self) -> Result<Vec<Weather>, CrateError> {
        let mut statement = self.conn.prepare("SELECT id, year, day, hour, icon, precip_intensity, precip_probability, temp, wind_speed, wind_gust, humidity, precip_type FROM weather ORDER BY id ASC")?;
        let weathers = statement.query_map(NO_PARAMS, |row| {
            return Ok(DbManager::build_weather(row));
        })?
            .map(|weather| weather.unwrap())
            .collect();

        return Ok(weathers);
    }

    fn build_weather(row: &Row) -> Weather {
        let icon: String = row.get_unwrap(4);
        return Weather::new(
            row.get_unwrap(0),
            row.get_unwrap(1),
            row.get_unwrap(2),
            row.get_unwrap(3),
            Icon::from_str(&icon),
            row.get_unwrap(5),
            row.get_unwrap(6),
            row.get_unwrap(7),
            row.get_unwrap(8),
            row.get_unwrap(9),
            row.get_unwrap(10),
            row.get_unwrap(11)
        );
    }

    fn build_prediction(row: &Row) -> Prediction {
        let icon: String = row.get_unwrap(8);
        return Prediction::new(
            row.get_unwrap(0),
            row.get_unwrap(1),
            row.get_unwrap(2),
            row.get_unwrap(3),
            row.get_unwrap(4),
            row.get_unwrap(5),
            row.get_unwrap(6),
            row.get_unwrap(7),
            Icon::from_str(&icon),
            row.get_unwrap(9),
            row.get_unwrap(10),
            row.get_unwrap(11),
            row.get_unwrap(12),
            row.get_unwrap(13),
            row.get_unwrap(14),
            row.get_unwrap(15)
        );
    }

    fn insert_weather(transaction: &Transaction, weather: &Weather) -> Result<(), CrateError> {
        let params = params![weather.id, weather.year, weather.day, weather.hour, weather.icon, weather.precip_intensity, weather.precip_probability, weather.temp, weather.wind_speed, weather.wind_gust, weather.humidity, weather.precip_type];
        transaction.execute("INSERT INTO weather (id, year, day, hour, icon, precip_intensity, precip_probability, temp, wind_speed, wind_gust, humidity, precip_type) VALUES (?,?,?,?,?,?,?,?,?,?,?,?)", params)?;

        Ok(())
    }

    fn insert_prediction(transaction: &Transaction, origin: &Weather, target: &Weather, hour_diff: usize) -> Result<(), CrateError> {
        let params = params![target.id, origin.year, origin.day, origin.hour, target.year, target.day, target.hour, hour_diff as u8, target.icon, target.precip_intensity, target.precip_probability, target.temp, target.wind_speed, target.wind_gust, target.humidity, target.precip_type];
        transaction.execute("INSERT INTO prediction (id, reading_year, reading_day, reading_hour, prediction_year, prediction_day, prediction_hour, hour_diff, icon, precip_intensity, precip_probability, temp, wind_speed, wind_gust, humidity, precip_type) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)", params)?;

        Ok(())
    }
}

trait UserVersion {
    fn get_user_version(&self) -> Result<usize, Error>;

    fn set_user_version(&mut self, version: usize) -> Result<(), Error>;
}

impl UserVersion for Connection {
    fn get_user_version(&self) -> Result<usize, Error> {
        self.query_row("PRAGMA user_version", NO_PARAMS, |row| row.get(0).map(|ver: i64| ver as usize))
    }

    fn set_user_version(&mut self, version: usize) -> Result<(), Error> {
        self.execute(&format!("PRAGMA user_version = {}", version), NO_PARAMS).and_then(|_| Ok(()))
    }
}