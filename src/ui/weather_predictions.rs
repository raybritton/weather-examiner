use crate::app::WeatherApp;
use crate::Error;
use std::io::stdout;
use crossterm::ExecutableCommand;
use crossterm::style::{Print, Color, SetBackgroundColor};
use crossterm::event::KeyCode;
use crate::ui::ui_section::UiSection;
use crate::ui::utils::{print_styled, print_first_last_reading};
use std::convert::TryInto;
use chrono::{NaiveDateTime, Datelike, Timelike};
use crate::extensions::Utils;
use std::time::Duration;

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

    fn print_temp_row(&self, data: &Vec<f64>, skip: usize, take: usize) -> Result<(), Error> {
        self.print_row(
            "Temp    ",
            HEADER_COLOR,
            data.iter().skip(skip).take(take).cloned().collect(),
            |val| format!("{: <3.0}   ", val),
            |_| Ok(()),
        )
    }

    fn print_prob_row(&self, data: &Vec<usize>, skip: usize, take: usize) -> Result<(), Error> {
        self.print_row(
            "P. Prob ",
            HEADER_COLOR,
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

    fn print_amt_row(&self, data: &Vec<f64>, skip: usize, take: usize) -> Result<(), Error> {
        self.print_row(
            "P. Amt  ",
            HEADER_COLOR,
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

    fn print_type_row(&self, data: &Vec<String>, skip: usize, take: usize) -> Result<(), Error> {
        self.print_row(
            "Precip  ",
            HEADER_COLOR,
            data.iter().skip(skip).take(take).cloned().collect(),
            |val| format!("{: <5} ", val),
            |_| Ok(()),
        )
    }

    fn print_gust_row(&self, data: &Vec<f64>, skip: usize, take: usize) -> Result<(), Error> {
        self.print_row(
            "Wnd Gst ",
            HEADER_COLOR,
            data.iter().skip(skip).take(take).cloned().collect(),
            |val| format!("{: <3.0}   ", val),
            |_| Ok(()),
        )
    }

    fn print_speed_row(&self, data: &Vec<f64>, skip: usize, take: usize) -> Result<(), Error> {
        self.print_row(
            "Wnd Spd ",
            HEADER_COLOR,
            data.iter().skip(skip).take(take).cloned().collect(),
            |val| format!("{: <3.0}   ", val),
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
                print_styled("\n\nOutside of data range\n\n", Color::Red, false)?;

                std::thread::sleep(Duration::from_millis(500));

                self.wait_for_char("\n\nPress any key to continue\n")?;

                return Ok(());
            } else {
                let reading = app.get_reading_with_predictions(selected_date.year() as u16, selected_date.ordinal() as u16, selected_date.hour() as u8)?;

                self.reset(self.reset_pos)?;

                stdout()
                    .execute(Print("\nViewing  "))?;

                print_styled(&format!("{}", selected_date.format("%a %Y-%m-%d %H:00")), Color::White, true)?;

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

                let mut speeds: Vec<f64> = reading.1.iter().map(|p| p.wind_speed).collect();
                speeds.insert(0, reading.0.wind_speed);

                let mut gusts: Vec<f64> = reading.1.iter().map(|p| p.wind_gust).collect();
                gusts.insert(0, reading.0.wind_gust);

                print_styled(&format!("\n\n        Time  {}", titles), HEADER_COLOR, false)?;
                self.print_temp_row(&temps, 0, 24)?;
                self.print_prob_row(&probs, 0, 24)?;
                self.print_amt_row(&amts, 0, 24)?;
                self.print_type_row(&types, 0, 24)?;
                self.print_speed_row(&speeds, 0, 24)?;
                self.print_gust_row(&gusts, 0, 24)?;

                print_styled(&format!("\n\n        {}", titles2), HEADER_COLOR, false)?;
                self.print_temp_row(&temps, 24, 24)?;
                self.print_prob_row(&probs, 24, 24)?;
                self.print_amt_row(&amts, 24, 24)?;
                self.print_type_row(&types, 24, 24)?;
                self.print_speed_row(&speeds, 24, 24)?;
                self.print_gust_row(&gusts, 24, 24)?;

                print_styled("\n\n(◄) Previous hour\n(►) Next hour\n(▲) Previous day\n(▼) Next day\n(esc) Go back", Color::Grey, false)?;

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