use crate::ui::ui_section::UiSection;
use crate::app::WeatherApp;
use crate::ui::weather_predictions::WeatherPredictions;
use crate::ui::weather_view::WeatherView;
use crate::Error;
use crate::ui::import_data::ImportData;
use crate::ui::missing_records::MissingRecords;

pub struct MainMenu {
    reset_pos: (u16, u16)
}

impl MainMenu {
    pub fn new(reset_pos: (u16, u16)) -> MainMenu {
        return MainMenu {
            reset_pos
        };
    }
}


impl UiSection for MainMenu {
    fn run(&mut self, app: &mut WeatherApp) -> Result<(), Error> {
        loop {
            self.reset(self.reset_pos)?;

            let menu_options = vec![
                "Import data",
                "Check for missing records",
                "Checking reading for specific point",
                "View predictions for specific point"
            ];

            let input = self.menu(menu_options, true)?;

            match input {
                0 => break,
                1 => ImportData::new(self.reset_pos).run(app)?,
                2 => MissingRecords::new(self.reset_pos).run(app)?,
                3 => WeatherView::new(self.reset_pos).run(app)?,
                4 => WeatherPredictions::new(self.reset_pos).run(app)?,
                _ => {}
            }
        }

        Ok(())
    }
}