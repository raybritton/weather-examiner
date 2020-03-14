use crate::ui::ui_section::UiSection;
use crate::Error;
use crate::app::WeatherApp;
use chrono::NaiveDateTime;
use crate::extensions::Utils;
use crate::ui::utils::{print_styled, print_first_last_reading};
use crossterm::style::{Color, SetBackgroundColor, Print};
use std::io::stdout;
use crossterm::ExecutableCommand;
use std::convert::TryInto;
use std::time::Duration;
use crossterm::event::KeyCode;
use crate::models::SimpleDate;

const HEADER_COLOR: Color = Color::Cyan;

pub struct DayView {
    reset_pos: (u16, u16)
}

impl DayView {
    pub fn new(reset_pos: (u16, u16)) -> DayView {
        return DayView {
            reset_pos
        };
    }

    fn print_temp_row(&self, data: Vec<f64>) -> Result<(), Error> {
        self.print_row(
            "Temp    ",
            HEADER_COLOR,
            data,
            |val| format!("{: <3.0}   ", val),
            |_| Ok(()),
        )
    }

    fn print_prob_row(&self, data: Vec<usize>) -> Result<(), Error> {
        self.print_row(
            "P. Prob ",
            HEADER_COLOR,
            data,
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

    fn print_amt_row(&self, data: Vec<f64>) -> Result<(), Error> {
        self.print_row(
            "P. Amt  ",
            HEADER_COLOR,
            data,
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

    fn print_gust_row(&self, data: Vec<f64>) -> Result<(), Error> {
        self.print_row(
            "Wnd Gst ",
            HEADER_COLOR,
            data,
            |val| format!("{: <3.0}   ", val),
            |_| Ok(()),
        )
    }

    fn print_speed_row(&self, data: Vec<f64>) -> Result<(), Error> {
        self.print_row(
            "Wnd Spd ",
            HEADER_COLOR,
            data,
            |val| format!("{: <3.0}   ", val),
            |_| Ok(()),
        )
    }
}

impl UiSection for DayView {
    fn run(&mut self, app: &mut WeatherApp) -> Result<(), Error> {
        self.reset(self.reset_pos)?;
        self.reset_pos = crossterm::cursor::position()?;

        let (first, last) = print_first_last_reading("View specific reading predictions\n", app)?;

        let mut selected_date: NaiveDateTime = self.input_year_day()?.try_into()?;

        loop {
            if selected_date < first.date() || selected_date > last.date() {
                print_styled("\n\nOutside of data range\n\n", Color::Red, false)?;

                std::thread::sleep(Duration::from_millis(500));

                self.wait_for_char("\n\nPress any key to continue\n")?;

                return Ok(());
            } else {
                let start: SimpleDate = selected_date.into();
                let mut end = start.clone();
                end.hour = 23;

                self.reset(self.reset_pos)?;

                stdout()
                    .execute(Print("\nViewing  "))?;

                print_styled(&format!("{}", selected_date.format("%a %Y-%m-%d")), Color::White, true)?;

                let readings = app.get_readings_over_range(start, end)?;

                let titles = (0..=23).map(|num| format!("{: <2}    ", num)).collect::<Vec<String>>().join("");
                print_styled(&format!("\n\n        {}", titles), HEADER_COLOR, false)?;

                let temps: Vec<f64> = readings.iter().map(|p| p.temp).collect();
                let probs: Vec<usize> = readings.iter().map(|p| (p.precip_probability * 100.) as usize).collect();
                let amts: Vec<f64> = readings.iter().map(|p| p.precip_intensity).collect();
                let speeds: Vec<f64> = readings.iter().map(|p| p.wind_speed).collect();
                let gusts: Vec<f64> = readings.iter().map(|p| p.wind_gust).collect();

                self.print_temp_row(temps)?;
                self.print_prob_row(probs)?;
                self.print_amt_row(amts)?;
                self.print_speed_row(speeds)?;
                self.print_gust_row(gusts)?;

                print_styled("\n\n(▲) Previous day\n(▼) Next day\n(esc) Go back", Color::Grey, false)?;

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
                        _ => {}
                    }
                }
            }
        }
    }
}