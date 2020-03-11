use crate::app::WeatherApp;
use crate::Error;
use std::io::{stdout, Write};
use crossterm::{ExecutableCommand, QueueableCommand};
use crossterm::style::{SetForegroundColor, Print, Color};
use crossterm::event::KeyCode;
use crate::ui::ui_section::UiSection;
use crate::ui::utils::{print_styled, print_first_last_reading};
use std::convert::TryInto;
use chrono::{NaiveDateTime, Datelike, Timelike};
use crate::extensions::Utils;

pub struct WeatherPredictions {
    reset_pos: (u16, u16)
}

impl WeatherPredictions {
    pub fn new(reset_pos: (u16, u16)) -> WeatherPredictions {
        return WeatherPredictions {
            reset_pos
        };
    }
}

impl UiSection for WeatherPredictions {
    fn run(&mut self, app: &mut WeatherApp) -> Result<(), Error> {
        self.reset(self.reset_pos)?;
        self.reset_pos = crossterm::cursor::position()?;

        let (first, last) = print_first_last_reading("View specific reading predictions\n", app)?;

        let mut selected_date: NaiveDateTime = self.input_year_day_hour()?.try_into()?;

        loop {
            if selected_date < first.date() || selected_date > last.date() {
                print_styled("Outside of data range", Color::Red, false)?;

                self.wait_for_char("\n\nPress any key to continue\n")?;

                return Ok(());
            } else {
                let reading = app.get_reading_with_predictions(selected_date.year() as u16, selected_date.ordinal0() as u16, selected_date.hour() as u8)?;

                self.reset(self.reset_pos)?;

                stdout()
                    .execute(Print("\nViewing  "))?;

                print_styled(&format!("{} {: >3} {: >2}\n", selected_date.year(), selected_date.ordinal0(), selected_date.hour()), Color::White, true)?;

                let titles = (1..=23).map(|num| format!("{: <2}    ", num)).collect::<Vec<String>>().join("");
                let titles2 = (24..=47).map(|num| format!("{: <2}    ", num)).collect::<Vec<String>>().join("");

                stdout()
                    .queue(Print("\n"))?
                    .queue(SetForegroundColor(Color::DarkCyan))?
                    .queue(Print(format!("        Time  {}", titles)))?
                    .queue(SetForegroundColor(Color::White))?;

                let temps = reading.1.iter().take(23).map(|prediction| format!("{: <3.0}   ", prediction.temp)).collect::<Vec<String>>().join("");
                let p_prob = reading.1.iter().take(23).map(|prediction| format!("{: <3}   ", (prediction.precip_probability * 100.) as usize)).collect::<Vec<String>>().join("");
                let p_iten = reading.1.iter().take(23).map(|prediction| format!("{:.1}   ", prediction.precip_intensity)).collect::<Vec<String>>().join("");
                let precip = reading.1.iter().take(23).map(|prediction| format!("{: <5} ", prediction.precip_type.as_ref().unwrap_or(&String::from("-")))).collect::<Vec<String>>().join("");

                stdout()
                    .queue(SetForegroundColor(Color::DarkCyan))?
                    .queue(Print("\nTemp    "))?
                    .queue(SetForegroundColor(Color::White))?
                    .queue(Print(format!("{: <5.0} {}", reading.0.temp, temps)))?
                    .queue(SetForegroundColor(Color::DarkCyan))?
                    .queue(Print("\nP. Prob "))?
                    .queue(SetForegroundColor(Color::White))?
                    .queue(Print(format!("{: <5} {}", (reading.0.precip_probability * 100.) as usize, p_prob)))?
                    .queue(SetForegroundColor(Color::DarkCyan))?
                    .queue(Print("\nP. Amt  "))?
                    .queue(SetForegroundColor(Color::White))?
                    .queue(Print(format!("{: <5} {}", reading.0.precip_intensity, p_iten)))?
                    .queue(SetForegroundColor(Color::DarkCyan))?
                    .queue(Print("\nPrecip  "))?
                    .queue(SetForegroundColor(Color::White))?
                    .queue(Print(format!("{: <5} {}", reading.0.precip_type.as_ref().unwrap_or(&String::from("-")), precip)))?;

                let temps = reading.1.iter().skip(23).take(24).map(|prediction| format!("{: <3.0}   ", prediction.temp)).collect::<Vec<String>>().join("");
                let p_prob = reading.1.iter().skip(23).take(24).map(|prediction| format!("{: <3}   ", (prediction.precip_probability * 100.) as usize)).collect::<Vec<String>>().join("");
                let p_iten = reading.1.iter().skip(23).take(24).map(|prediction| format!("{:.1}   ", prediction.precip_intensity)).collect::<Vec<String>>().join("");
                let precip = reading.1.iter().skip(23).take(24).map(|prediction| format!("{: <5} ", prediction.precip_type.as_ref().unwrap_or(&String::from("-")))).collect::<Vec<String>>().join("");

                stdout()
                    .queue(Print("\n\n"))?
                    .queue(SetForegroundColor(Color::DarkCyan))?
                    .queue(Print(format!("        {}", titles2)))?
                    .queue(SetForegroundColor(Color::White))?;

                stdout()
                    .queue(SetForegroundColor(Color::DarkCyan))?
                    .queue(Print("\nTemp    "))?
                    .queue(SetForegroundColor(Color::White))?
                    .queue(Print(temps))?
                    .queue(SetForegroundColor(Color::DarkCyan))?
                    .queue(Print("\nP. Prob "))?
                    .queue(SetForegroundColor(Color::White))?
                    .queue(Print(p_prob))?
                    .queue(SetForegroundColor(Color::DarkCyan))?
                    .queue(Print("\nP. Amt  "))?
                    .queue(SetForegroundColor(Color::White))?
                    .queue(Print(p_iten))?
                    .queue(SetForegroundColor(Color::DarkCyan))?
                    .queue(Print("\nPrecip  "))?
                    .queue(SetForegroundColor(Color::White))?
                    .queue(Print(precip))?;

                stdout().flush()?;

                stdout()
                    .execute(Print("\n\n(◄) Previous slot\n(►) Next slot\n(esc) Go back"))?;

                loop {
                    let char = self.wait_for_char_no_delay()?;

                    match char {
                        KeyCode::Esc => return Ok(()),
                        KeyCode::Left => {
                            selected_date = selected_date.minus_one_hour();
                            break;
                        }
                        KeyCode::Right => {
                            selected_date = selected_date.plus_one_hour();
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}