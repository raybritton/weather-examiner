use app_dirs2::{AppInfo, app_root, AppDataType};
use clap::{App, Arg, crate_description, crate_authors, crate_name, crate_version};
use simplelog::{SimpleLogger, ConfigBuilder};
use log::{LevelFilter, error};
use crate::app::WeatherApp;

mod app;

const APP_INFO: AppInfo = AppInfo {
    name:  "Weather",
    author: "Ray Britton"
};

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("db")
            .long("database")
            .short("d")
            .takes_value(true)
            .value_name("PATH")
            .multiple(false)
            .number_of_values(1)
            .help("Weather database file to use\nFile will be created if it doesn't exist\nIf not set this program will automatically generate one in the users data directory"))
        .get_matches();

    let config = ConfigBuilder::new()
        .set_thread_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .set_location_level(LevelFilter::Trace)
        .build();

    if let Err(err) = SimpleLogger::init(LevelFilter::Error, config) {
        eprintln!("Logger failed to initialise\nNo other errors will be printed\n{}", err);
    }

    let db_file = if matches.is_present("db") {
        matches.value_of("db").unwrap().to_owned()
    } else {
        match app_root(AppDataType::SharedData, &APP_INFO) {
            Ok(path) => {
                match path.to_str() {
                    None => {
                        error!("Unfortunately the shared data dir path contains invalid UTF-8\nTry setting a specific path with --database <PATH>");
                        return;
                    },
                    Some(data_dir) => data_dir.to_owned()
                }
            },
            Err(err) => {
                error!("Unable to access shared data dir: {}", err);
                error!("Try setting a specific path with --database <PATH>");
                return;
            },
        }
    };

    let app = WeatherApp::new(db_file);
    app.run();
}
