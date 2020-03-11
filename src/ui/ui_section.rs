use crate::Error;
use std::io::{stdout, stdin};
use crossterm::style::Print;
use crossterm::ExecutableCommand;
use crossterm::event::KeyCode;
use std::time::Duration;
use crossterm::event::Event::Key;
use crate::app::WeatherApp;
use log::error;
use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType};
use crate::models::SimpleDate;
use crate::ui::utils::consume_all_input;

pub trait UiSection {
    fn run(&mut self, app: &mut WeatherApp) -> Result<(), Error>;

    /// Read input once from stdin
    ///
    /// # Panics
    /// Will exit process if stdin can not be read from (via `process::exit`, not panic)
    ///
    /// # Returns
    /// Trimmed input
    ///
    fn read_input(&self, message: &str) -> Result<String, Error> {
        stdout()
            .execute(Print(message))?;
        let mut input = String::new();
        while input.trim().is_empty() {
            if let Err(err) = stdin().read_line(&mut input) {
                error!("Can not read input: {}", err);
                std::process::exit(1);
            }
        }
        return Ok(input.trim().to_owned());
    }

    /// Pauses for 300ms then waits for a single key press
    ///
    /// # Errors
    /// Unable to start/end raw mode
    /// Unable to read event
    ///
    /// # Returns
    /// KeyCode of key pressed
    ///
    fn wait_for_char(&self, message: &str) -> Result<KeyCode, Error> {
        std::thread::sleep(Duration::from_millis(300));
        stdout()
            .execute(Print(message))?;
        crossterm::terminal::enable_raw_mode()?;
        loop {
            let result = crossterm::event::read()?;
            if let Key(key) = result {
                crossterm::terminal::disable_raw_mode()?;
                return Ok(key.code);
            }
        }
    }

    /// Waits for a single key press
    ///
    /// # Errors
    /// Unable to start/end raw mode
    /// Unable to read event
    ///
    /// # Returns
    /// KeyCode of key pressed
    ///
    fn wait_for_char_no_delay(&self) -> Result<KeyCode, Error> {
        crossterm::terminal::enable_raw_mode()?;
        loop {
            let result = crossterm::event::read()?;
            if let Key(key) = result {
                crossterm::terminal::disable_raw_mode()?;
                return Ok(key.code);
            }
        }
    }

    /// Move cursor back to reset pos and clear all lines below
    ///
    fn reset(&self, reset_pos: (u16, u16)) -> Result<(), Error> {
        stdout()
            .execute(MoveTo(reset_pos.0, reset_pos.1))?
            .execute(Clear(ClearType::FromCursorDown))?;
        Ok(())
    }

    fn input_year_day_hour(&mut self) -> Result<SimpleDate, Error> {
        consume_all_input()?;

        let year = self.read_input("\n\nEnter year\n")?.parse()?;
        let day = self.read_input("\n\nEnter day\n")?.parse()?;
        let hour = self.read_input("\n\nEnter hour\n")?.parse()?;

        Ok(SimpleDate::new(year, day, hour))
    }
}