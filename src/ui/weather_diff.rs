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
use std::time::Duration;
use std::iter::Iterator;

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

        self.wait_for_char("Press any key")?;

        Ok(())
    }
}