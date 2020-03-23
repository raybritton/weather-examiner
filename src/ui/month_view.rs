use crate::ui::ui_section::UiSection;
use crate::Error;
use crate::app::WeatherApp;
use std::io::stdout;
use crossterm::style::{SetBackgroundColor, Color, Print};
use crossterm::ExecutableCommand;
use crate::ui::utils::{print_first_last_reading, print_styled, print_row_titles};
use chrono::{NaiveDateTime, Datelike, NaiveDate, NaiveTime, Timelike};
use std::time::Duration;
use crate::models::SimpleDate;
use crossterm::event::KeyCode;
use crate::extensions::{Utils, days_in_month};
use crate::min_max_avg::{avg_usize, max_usize, min_usize, avg_f64, min_f64, max_f64};

const HEADER_COLOR: Color = Color::Cyan;

pub struct MonthView {
    reset_pos: (u16, u16)
}

impl MonthView {
    pub fn new(reset_pos: (u16, u16)) -> MonthView {
        return MonthView {
            reset_pos
        };
    }
}

impl MonthView {
    fn print_temp_row(&self, data: &Vec<(f64, f64, f64)>, skip: usize, take: usize) -> Result<(), Error> {
        self.print_row(
            "Temp    ",
            HEADER_COLOR,
            data.iter().skip(skip).take(take).cloned().collect(),
            |val| format!("{: <3.0}/{: <3.0}/{: <3.0}  ", val.0, val.1, val.2),
            |_| Ok(()),
        )
    }

    fn print_prob_row(&self, data: &Vec<(usize, usize, usize)>, skip: usize, take: usize) -> Result<(), Error> {
        self.print_row(
            "P. Prob ",
            HEADER_COLOR,
            data.iter().skip(skip).take(take).cloned().collect(),
            |val| format!("{: <3}/{: <3}/{: <3}  ", val.0, val.1, val.2),
            |val| {
                let ansi = match val.1 {
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

    fn print_amt_row(&self, data: &Vec<(f64, f64, f64)>, skip: usize, take: usize) -> Result<(), Error> {
        self.print_row(
            "P. Amt  ",
            HEADER_COLOR,
            data.iter().skip(skip).take(take).cloned().collect(),
            |val| format!("{:.1}/{:.1}/{:.1}  ", val.0, val.1, val.2),
            |val| {
                let ansi = match val.1 {
                    d if d > 3.0 => 21,
                    1.0..=2.999 => 20,
                    0.3..=0.999 => 18,
                    _ => 16
                };
                stdout().execute(SetBackgroundColor(Color::AnsiValue(ansi)))?;
                Ok(())
            },
        )
    }
}

impl UiSection for MonthView {
    fn run(&mut self, app: &mut WeatherApp) -> Result<(), Error> {
        self.reset(self.reset_pos)?;

        self.reset_pos = crossterm::cursor::position()?;

        let (first, last) = print_first_last_reading("View specific reading predictions\n", app)?;

        let (year, month) = self.input_year_month()?;
        let mut selected_date = NaiveDateTime::new(NaiveDate::from_ymd(year as i32, month as u32, 1), NaiveTime::from_hms(0,0,0));

        loop {
            if selected_date < first.date() || selected_date > last.date() {
                print_styled("\n\nOutside of data range\n\n", Color::Red, false)?;

                std::thread::sleep(Duration::from_millis(500));

                self.wait_for_char("\n\nPress any key to continue\n")?;

                return Ok(());
            } else {
                let days_in_month = days_in_month(selected_date.month() as u8, selected_date.year() as u32);

                let start: SimpleDate = selected_date.into();
                let mut end = selected_date
                    .with_day(days_in_month as u32).expect("Invalid days_in_month")
                    .with_hour(23).unwrap()
                    .clone()
                    .into();

                self.reset(self.reset_pos)?;

                stdout()
                    .execute(Print("\nViewing  "))?;

                print_styled(&format!("{}", selected_date.format("%Y %b")), Color::White, true)?;

                stdout()
                    .execute(Print("\n(Min/Avg/Max)"))?;

                let readings = app.get_readings_over_range(start, end)?;

                let mut daily_temps = vec![];
                let mut daily_probs = vec![];
                let mut daily_amts = vec![];

                readings.iter().map(|p| p.temp)
                    .collect::<Vec<f64>>()
                    .chunks_exact(24)
                    .for_each(|temps| {
                        daily_temps.push((
                            min_f64(temps),
                            avg_f64(temps),
                            max_f64(temps)
                            ));
                    });

                readings.iter().map(|p| (p.precip_probability * 100.) as usize)
                    .collect::<Vec<usize>>()
                    .chunks_exact(24)
                    .for_each(|probs| {
                        daily_probs.push((
                            min_usize(probs),
                            avg_usize(probs),
                            max_usize(probs)
                        ));
                    });

                readings.iter().map(|p| p.precip_intensity)
                    .collect::<Vec<f64>>()
                    .chunks_exact(24)
                    .for_each(|amts| {
                        daily_amts.push((
                            min_f64(amts),
                            avg_f64(amts),
                            max_f64(amts)
                        ));
                    });

                print_row_titles(1, 11, 11, HEADER_COLOR)?;
                self.print_temp_row(&daily_temps, 0, 11)?;
                self.print_prob_row(&daily_probs, 0, 11)?;
                self.print_amt_row(&daily_amts, 0, 11)?;

                print_row_titles(12, 22, 11, HEADER_COLOR)?;
                self.print_temp_row(&daily_temps, 11, 11)?;
                self.print_prob_row(&daily_probs, 11, 11)?;
                self.print_amt_row(&daily_amts, 11, 11)?;

                let count = (days_in_month - 22) as usize;
                print_row_titles(23, days_in_month as usize, 11, HEADER_COLOR)?;
                self.print_temp_row(&daily_temps, 22, count)?;
                self.print_prob_row(&daily_probs, 22, count)?;
                self.print_amt_row(&daily_amts, 22, count)?;

                print_styled("\n\n(▲) Previous month\n(▼) Next month\n(esc) Go back", Color::Grey, false)?;

                loop {
                    let char = self.wait_for_char_no_delay()?;

                    match char {
                        KeyCode::Esc => return Ok(()),
                        KeyCode::Up => {
                            selected_date = selected_date.minus_one_month();
                            break;
                        }
                        KeyCode::Down => {
                            selected_date = selected_date.plus_one_month();
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}