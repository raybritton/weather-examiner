use crate::ui::ui_section::UiSection;
use crate::Error;
use crate::app::WeatherApp;

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

impl UiSection for MonthView {
    fn run(&mut self, app: &mut WeatherApp) -> Result<(), Error> {
        self.reset(self.reset_pos)?;

        self.wait_for_char("\n\n\nPress any key to continue\n")?;

        Ok(())
    }
}