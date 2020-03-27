use crate::ui::ui_section::UiSection;
use crate::Error;
use crate::app::WeatherApp;
use chrono::NaiveDateTime;
use crate::extensions::Utils;
use crate::ui::utils::{print_styled, print_first_last_reading};
use crossterm::style::{Color, Print};
use std::io::stdout;
use crossterm::ExecutableCommand;
use std::convert::TryInto;
use std::time::Duration;
use crossterm::event::KeyCode;
use crate::models::SimpleDate;

const HEADER_COLOR: Color = Color::Cyan;

pub struct WeatherDiff {
    reset_pos: (u16, u16)
}

impl WeatherDiff {
    pub fn new(reset_pos: (u16, u16)) -> WeatherDiff {
        return WeatherDiff {
            reset_pos
        };
    }
}

impl WeatherDiff {}

impl UiSection for WeatherDiff {
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
                let start: SimpleDate = selected_date.into();
                let mut end = start.clone();
                end.hour = 23;

                self.reset(self.reset_pos)?;

                stdout()
                    .execute(Print("\nViewing  "))?;

                print_styled(&format!("{}", selected_date.format("%a %Y-%m-%d")), Color::White, true)?;

                let readings = app.get_readings_over_range(start, end)?;

                

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