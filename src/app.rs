pub struct WeatherApp {
    db_file: String
}

impl WeatherApp {
    pub fn new<T: Into<String>>(db_file: T) -> WeatherApp {
        return WeatherApp {
            db_file: db_file.into()
        }
    }
}

impl WeatherApp {
    pub fn run(&self) {
        loop {

        }
    }
}