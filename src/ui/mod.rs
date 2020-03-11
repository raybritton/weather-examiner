use crate::app::WeatherApp;
use log::error;
use std::io::{stdout, stdin, Write};
use crossterm::{ExecutableCommand, QueueableCommand};
use crossterm::style::{Print, Color};
use crate::Error;
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::MoveTo;
use crossterm::event::KeyCode;
use crossterm::event::Event::Key;
use crate::ui::weather_predictions::WeatherPredictions;
use crate::ui::ui_section::UiSection;
use crate::ui::weather_view::WeatherView;
use crate::ui::utils::{print_styled, consume_all_input};

mod weather_predictions;
mod ui_section;
mod utils;
mod weather_view;

pub struct Ui {
    app: WeatherApp,
    reset_pos: Option<(u16, u16)>
}

impl Ui {
    pub fn new(app: WeatherApp) -> Ui {
        return Ui {
            app,
            reset_pos: None
        };
    }
}

impl Ui {
    pub fn run(&mut self) -> Result<(), Error> {

        stdout()
            .execute(crossterm::terminal::SetSize(160, 40))?
            .execute(Clear(ClearType::All))?
            .execute(MoveTo(0, 0))?;

        print_styled("Weather prediction examiner\n\n", Color::Cyan, true)?;

        self.reset_pos = Some(crossterm::cursor::position()?);

        loop {
            self.reset()?;

            stdout()
                .execute(Print("1) Import data\n"))?
                .execute(Print("2) Check for missing records\n"))?
                .execute(Print("3) Checking reading for specific point\n"))?
                .execute(Print("4) View predictions for specific point\n"))?
                .execute(Print("\n0) Exit\n"))?;

            let input = self.wait_for_char("")?;

            let pos = self.reset_pos.expect("No reset found when starting weather predictions");

            match input {
                KeyCode::Char('0') | KeyCode::Esc => break,
                KeyCode::Char('1') => self.import_data()?,
                KeyCode::Char('2') => self.check_for_missing_records()?,
                KeyCode::Char('3') => WeatherView::new(pos).run(&mut self.app)?,
                KeyCode::Char('4') => WeatherPredictions::new(pos).run(&mut self.app)?,
                _ => {}
            }
        }

        Ok(())
    }

    fn import_data(&mut self) -> Result<(), Error> {
        self.reset()?;

        let dir = self.read_input("Enter a directory to import from\n")?;

        stdout().execute(Print("\nImporting\n"))?;

        self.app.import_data(dir)?;

        self.wait_for_char("\nDone\nPress any key to continue\n")?;

        Ok(())
    }

    fn check_for_missing_records(&mut self) -> Result<(), Error> {
        self.reset()?;

        stdout()
            .execute(Print("Searching..."))?;

        let results = self.app.check_for_missing_data()?;

        self.reset()?;

        stdout()
            .execute(Print("Missing:\n"))?;

        if results.is_empty() {
            stdout()
                .queue(Print("None"))?;
        }
        for result in results {
            stdout()
                .queue(Print(format!("{}", result.format("%Y %j %H"))))?;
        }

        stdout().flush()?;

        self.wait_for_char("\n\n\nPress any key to continue\n")?;

        Ok(())
    }

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
        consume_all_input()?;
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

    fn reset(&self) -> Result<(), Error> {
        if let Some(pos) = self.reset_pos {
            stdout()
                .execute(MoveTo(pos.0, pos.1))?
                .execute(Clear(ClearType::FromCursorDown))?;
        }

        Ok(())
    }
}