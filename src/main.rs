use app_dirs2::{AppInfo, app_root, AppDataType};
use clap::{App, Arg, crate_description, crate_authors, crate_name, crate_version};
use simplelog::{SimpleLogger, ConfigBuilder};
use log::{LevelFilter, error, trace, info};
use crate::app::WeatherApp;
use crate::db_manager::DbManager;
use crate::ui::Ui;

pub type Error = Box<dyn std::error::Error>;

mod app;
mod db_manager;
mod models;
mod templates;
mod ui;
mod extensions;
mod min_max_avg;

const APP_INFO: AppInfo = AppInfo {
    name: "Weather",
    author: "Ray Britton",
};

fn main() -> Result<(), Error> {
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
        .arg(Arg::with_name("verbose")
            .takes_value(false)
            .short("v")
            .long("verbose")
            .help("Set verbosity of program (between 0 and 3)")
            .required(false)
            .multiple(true))
        .arg(Arg::with_name("update")
            .takes_value(true)
            .long("update")
            .value_name("PATH")
            .help("Update DB with all json files at path and exit")
            .multiple(false)
            .number_of_values(1))
        .arg(Arg::with_name("path")
            .takes_value(false)
            .long("path")
            .help("Print database path and exit")
            .multiple(false))
        .get_matches();

    let verbosity = matches.occurrences_of("verbose");

    let config = ConfigBuilder::new()
        .set_thread_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .set_location_level(LevelFilter::Error)
        .build();

    let log_level = int_to_log_level(verbosity);

    if let Err(err) = SimpleLogger::init(log_level, config) {
        eprintln!("Logger failed to initialise\nNo other errors will be printed\n{}", err);
    }

    let db_file = if matches.is_present("db") {
        matches.value_of("db").unwrap().to_owned()
    } else {
        match app_root(AppDataType::UserData, &APP_INFO) {
            Ok(path) => {
                match path.to_str() {
                    None => {
                        error!("Unfortunately the user data dir path contains invalid UTF-8\nTry setting a specific path with --database <PATH>");
                        std::process::exit(1);
                    }
                    Some(data_dir) => format!("{}/weather.db", data_dir.to_owned())
                }
            }
            Err(err) => {
                error!("Unable to access user data dir: {}", err);
                error!("Try setting a specific path with --database <PATH>");
                std::process::exit(1);
            }
        }
    };

    let mut db_manager = DbManager::new(db_file.clone())?;

    db_manager.init()?;

    let mut app = WeatherApp::new(db_manager);

    if let Some(update_dir) = matches.value_of("update") {
        trace!("Importing");
        app.import_data(update_dir.to_string())?;
        info!("Done");
    } else if matches.is_present("path") {
        println!("{}", db_file);
    } else {
        let mut ui = Ui::new(app);

        ui.run()?;
    }

    Ok(())
}

fn int_to_log_level(count: u64) -> log::LevelFilter {
    return match count.min(3) {
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        3 => log::LevelFilter::Trace,
        _ => log::LevelFilter::Error
    };
}

