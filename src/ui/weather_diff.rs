use crate::app::WeatherApp;
use crate::Error;
use crossterm::style::Color;
use crate::ui::ui_section::UiSection;

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