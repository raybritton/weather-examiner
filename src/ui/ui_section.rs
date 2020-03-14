use crate::Error;
use std::io::{stdout, stdin, Write};
use crossterm::style::{Print, Color};
use crossterm::{ExecutableCommand, QueueableCommand, cursor};
use crossterm::event::KeyCode;
use crossterm::event::Event::Key;
use crate::app::WeatherApp;
use log::error;
use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType};
use crate::models::SimpleDate;
use crate::ui::utils::{consume_all_input, print_styled, print_styled_list};
use crate::extensions::MapToUnit;
use std::any::Any;

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
            .execute(cursor::Show)?
            .execute(Print(message))?;
        let mut input = String::new();
        while input.trim().is_empty() {
            if let Err(err) = stdin().read_line(&mut input) {
                error!("Can not read input: {}", err);
                std::process::exit(1);
            }
        }
        stdout()
            .execute(cursor::Hide)?;
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
        consume_all_input()?;
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
    /// # Errors
    /// Unable to move cursor
    /// Unable to clear screen
    ///
    fn reset(&self, reset_pos: (u16, u16)) -> Result<(), Error> {
        stdout()
            .execute(MoveTo(reset_pos.0, reset_pos.1))?
            .execute(Clear(ClearType::FromCursorDown))?;
        Ok(())
    }

    fn input_year_day_hour(&mut self) -> Result<SimpleDate, Error> {
        consume_all_input()?;
        stdout()
            .execute(cursor::Show)?;

        let year = self.read_input("\n\nEnter year\n")?.parse()?;
        let day = self.read_input("\n\nEnter day\n")?.parse()?;
        let hour = self.read_input("\n\nEnter hour\n")?.parse()?;

        stdout()
            .execute(cursor::Hide)?;

        Ok(SimpleDate::new(year, day, hour))
    }

    fn input_year_day(&mut self) -> Result<SimpleDate, Error> {
        consume_all_input()?;
        stdout()
            .execute(cursor::Show)?;

        let year = self.read_input("\n\nEnter year\n")?.parse()?;
        let day = self.read_input("\n\nEnter day\n")?.parse()?;

        stdout()
            .execute(cursor::Hide)?;

        Ok(SimpleDate::new(year, day, 0))
    }

    fn input_year_month(&mut self) -> Result<(u16, u8), Error> {
        consume_all_input()?;
        stdout()
            .execute(cursor::Show)?;

        let year = self.read_input("\n\nEnter year\n")?.parse()?;
        let month = self.read_input("\n\nEnter month\n")?.parse()?;

        stdout()
            .execute(cursor::Hide)?;

        Ok((year, month))
    }

    /// Show a menu of options
    ///
    /// If exit is true then a final option of 'Exit' will be added
    ///
    /// # Errors
    /// Unable to print text
    /// Unable to read event
    ///
    /// # Returns
    /// 0 - exit (if enabled)
    /// 1 - 9 for selected option
    ///
    fn menu(&mut self, options: Vec<&str>, exit: bool) -> Result<usize, Error> {
        options.iter()
            .enumerate()
            .try_for_each(|(i, option)|
                stdout()
                    .queue(Print(format!("{}) {}\n", i + 1, option)))
                    .map_to_unit()
            )?;

        if exit {
            stdout().queue(Print("\nesc) Exit\n"))?;
        }

        stdout().flush()?;

        loop {
            let input = self.wait_for_char("")?;

            if input == KeyCode::Esc {
                return Ok(0);
            } else if let KeyCode::Char(chr) = input {
                if let Some(num) = chr.to_digit(10).map(|num| num as usize) {
                    if num <= options.len() {
                        return Ok(num);
                    }
                }
            }
        }
    }

    fn print_row<D, F, S>(&self, title: &str, header_color: Color, data: Vec<D>, formatter: F, styler: S) -> Result<(), Error> where
        D: Any,
        F: Fn(D) -> String,
        S: Fn(&D) -> Result<(), Error>
    {
        print_styled(&format!("\n{}", title), header_color, false)?;
        print_styled_list(data, formatter, styler)?;

        Ok(())
    }
}