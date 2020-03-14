use crossterm::style::{Color, SetAttribute, SetForegroundColor, Print, Attribute, SetBackgroundColor};
use std::io::stdout;
use crate::Error;
use crossterm::ExecutableCommand;
use crate::models::Weather;
use crate::app::WeatherApp;
use std::time::Duration;
use crate::extensions::MapToUnit;
use std::any::Any;

pub fn print_styled(msg: &str, color: Color, bold: bool) -> Result<(), Error> {
    if bold {
        stdout().execute(SetAttribute(Attribute::Bold))?;
    }
    stdout().execute(SetForegroundColor(color))?
        .execute(Print(msg))?
        .execute(SetAttribute(Attribute::NormalIntensity))?
        .execute(SetForegroundColor(Color::White))?;

    Ok(())
}

pub fn print_styled_list<D, F, S>(data: Vec<D>, formatter: F, styler: S) -> Result<(), Error> where
    D: Any,
    F: Fn(D) -> String,
    S: Fn(&D) -> Result<(), Error>
{
    data.into_iter()
        .try_for_each(|item| {
            styler(&item)?;
            stdout()
                .execute(Print(formatter(item)))?
                .execute(SetAttribute(Attribute::NormalIntensity))?
                .execute(SetForegroundColor(Color::White))?
                .execute(SetBackgroundColor(Color::Black))
                .map_err(|e| Error::from(e))
                .map_to_unit()
        })
}

pub fn print_first_last_reading(msg: &str, app: &mut WeatherApp) -> Result<(Weather, Weather), Error> {
    let first = app.get_first_reading()?;
    let last = app.get_last_reading()?;

    print_styled(msg, Color::Cyan, false)?;

    stdout()
        .execute(Print(format!("Earliest: {} {: >3} {: >2}\n", first.year, first.day, first.hour)))?
        .execute(Print(format!("Latest:   {} {: >3} {: >2}\n", last.year, last.day, last.hour)))?;

    std::thread::sleep(Duration::from_millis(300));

    return Ok((first, last));
}

pub fn consume_all_input() -> Result<(), Error> {
    let duration = Duration::from_millis(10);
    while crossterm::event::poll(duration)? {
        crossterm::event::read()?;
    }
    Ok(())
}