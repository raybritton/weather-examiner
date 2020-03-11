use crate::ui::ui_section::UiSection;
use crate::app::WeatherApp;
use crate::Error;
use std::io::stdout;
use crossterm::style::{SetForegroundColor, Color, Print};
use crossterm::ExecutableCommand;
use chrono::{NaiveDateTime, Datelike, Timelike};
use crate::ui::utils::print_first_last_reading;
use std::convert::TryInto;

pub struct WeatherView {
    reset_pos: (u16, u16)
}

impl WeatherView {
    pub fn new(reset_pos: (u16, u16)) -> WeatherView {
        return WeatherView {
            reset_pos
        };
    }
}

impl UiSection for WeatherView {
    fn run(&mut self, app: &mut WeatherApp) -> Result<(), Error> {
        self.reset(self.reset_pos)?;
        self.reset_pos = crossterm::cursor::position()?;

        let (first, last) = print_first_last_reading("View specific reading\n", app)?;

        let selected_date: NaiveDateTime = self.input_year_day_hour()?.try_into()?;

        if selected_date < first.date() || selected_date > last.date(){
            stdout()
                .execute(SetForegroundColor(Color::Red))?
                .execute(Print("Outside of data range"))?
                .execute(SetForegroundColor(Color::White))?;
        } else {
            let reading = app.get_reading(selected_date.year() as u16, selected_date.ordinal0() as u16, selected_date.hour() as u8)?;

            self.reset(self.reset_pos)?;

            stdout()
                .execute(Print(format!("{}", reading)))?;
        }

        self.wait_for_char("\n\nPress any key to continue\n")?;

        Ok(())
    }
}