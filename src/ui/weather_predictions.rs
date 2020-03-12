use crate::app::WeatherApp;
use crate::Error;
use std::io::{stdout, Write};
use crossterm::{ExecutableCommand, QueueableCommand};
use crossterm::style::{SetForegroundColor, Print, Color, SetAttribute, Attribute, SetBackgroundColor};
use crossterm::event::KeyCode;
use crate::ui::ui_section::UiSection;
use crate::ui::utils::{print_styled, print_first_last_reading, print_styled_list};
use std::convert::TryInto;
use chrono::{NaiveDateTime, Datelike, Timelike};
use crate::extensions::{Utils, MapToUnit};
use std::any::Any;

const HEADER_COLOR: Color = Color::Cyan;

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

impl WeatherPredictions {
    fn print_row<D, F, S>(title: &str, data: Vec<D>, formatter: F, styler: S) -> Result<(), Error> where
        D: Any,
        F: Fn(D) -> String,
        S: Fn(&D) -> Result<(), Error>
    {
        print_styled(&format!("\n{}", title), HEADER_COLOR, false)?;
        print_styled_list(data, formatter, styler)?;

        Ok(())
    }

    fn print_temp_row(data: &Vec<f64>, skip: usize, take: usize) -> Result<(), Error> {
        WeatherPredictions::print_row(
            "Temp    ",
            data.iter().skip(skip).take(take).cloned().collect(),
            |val| format!("{: <3.0}   ", val),
            |_| Ok(()),
        )
    }

    fn print_prob_row(data: &Vec<usize>, skip: usize, take: usize) -> Result<(), Error> {
        WeatherPredictions::print_row(
            "P. Prob ",
            data.iter().skip(skip).take(take).cloned().collect(),
            |val| format!("{: <3}   ", val),
            |val| {
                let ansi = match val {
                    90..=100 => 21,
                    70..=89 => 20,
                    50..=69 => 19,
                    30..=49 => 18,
                    _ => 16
                };
                stdout().execute(SetBackgroundColor(Color::AnsiValue(ansi)))?;
                Ok(())
            },
        )
    }

    fn print_amt_row(data: &Vec<f64>, skip: usize, take: usize) -> Result<(), Error> {
        WeatherPredictions::print_row(
            "P. Amt  ",
            data.iter().skip(skip).take(take).cloned().collect(),
            |val| format!("{:.1}   ", val),
            |val| {
                let ansi = match val {
                    d if d > &3.0 => 21,
                    1.0..=2.999 => 20,
                    0.3..=0.999 => 18,
                    _ => 16
                };
                stdout().execute(SetBackgroundColor(Color::AnsiValue(ansi)))?;
                Ok(())
            },
        )
    }

    fn print_type_row(data: &Vec<String>, skip: usize, take: usize) -> Result<(), Error> {
        WeatherPredictions::print_row(
            "Precip  ",
            data.iter().skip(skip).take(take).cloned().collect(),
            |val| format!("{: <5} ", val),
            |_| Ok(()),
        )
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
                let reading = app.get_reading_with_predictions(selected_date.year() as u16, selected_date.ordinal() as u16, selected_date.hour() as u8)?;

                self.reset(self.reset_pos)?;

                stdout()
                    .execute(Print("\nViewing  "))?;

                print_styled(&format!("{} {: >3} {: >2}\n", selected_date.year(), selected_date.ordinal(), selected_date.hour()), Color::White, true)?;

                let titles = (1..=23).map(|num| format!("{: <2}    ", num)).collect::<Vec<String>>().join("");
                let titles2 = (24..=47).map(|num| format!("{: <2}    ", num)).collect::<Vec<String>>().join("");

                let mut temps: Vec<f64> = reading.1.iter().map(|p| p.temp).collect();
                temps.insert(0, reading.0.temp);

                let mut probs: Vec<usize> = reading.1.iter().map(|p| (p.precip_probability * 100.) as usize).collect();
                probs.insert(0, (reading.0.precip_probability * 100.) as usize);

                let mut amts: Vec<f64> = reading.1.iter().map(|p| p.precip_intensity).collect();
                amts.insert(0, reading.0.precip_intensity);

                let mut types: Vec<String> = reading.1.iter().map(|p| p.precip_type.as_ref().unwrap_or(&String::from("-")).clone()).collect();
                types.insert(0, reading.0.precip_type.as_ref().unwrap_or(&String::from("-")).clone());

                print_styled(&format!("\n        Time  {}", titles), HEADER_COLOR, false)?;
                WeatherPredictions::print_temp_row(&temps, 0, 24)?;
                WeatherPredictions::print_prob_row(&probs, 0, 24)?;
                WeatherPredictions::print_amt_row(&amts, 0, 24)?;
                WeatherPredictions::print_type_row(&types, 0, 24)?;

                print_styled(&format!("\n\n        {}", titles2), HEADER_COLOR, false)?;
                WeatherPredictions::print_temp_row(&temps, 24, 24)?;
                WeatherPredictions::print_prob_row(&probs, 24, 24)?;
                WeatherPredictions::print_amt_row(&amts, 24, 24)?;
                WeatherPredictions::print_type_row(&types, 24, 24)?;

                stdout()
                    .execute(Print("\n\n(◄) Previous slot\n(►) Next slot\n(▲) Previous day\n(▼) Next day\n(esc) Go back"))?;

                loop {
                    let char = self.wait_for_char_no_delay()?;

                    match char {
                        KeyCode::Esc => return Ok(()),
                        KeyCode::Up => {
                            selected_date = selected_date.minus_one_day();
                            break;
                        }
                        KeyCode::Down => {
                            selected_date = selected_date.plus_one_day();
                            break;
                        }
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