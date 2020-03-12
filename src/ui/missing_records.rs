use crate::ui::ui_section::UiSection;
use std::io::{stdout, Write};
use crate::Error;
use crate::app::WeatherApp;
use crossterm::style::Print;
use crossterm::{ExecutableCommand, QueueableCommand};

pub struct MissingRecords {
    reset_pos: (u16, u16)
}

impl MissingRecords {
    pub fn new(reset_pos: (u16, u16)) -> MissingRecords {
        return MissingRecords {
            reset_pos
        };
    }
}

impl UiSection for MissingRecords {
    fn run(&mut self, app: &mut WeatherApp) -> Result<(), Error> {
        self.reset(self.reset_pos)?;

        stdout()
            .execute(Print("Searching..."))?;

        let results = app.check_for_missing_data()?;

        self.reset(self.reset_pos)?;

        stdout()
            .execute(Print("Missing:\n"))?;

        if results.is_empty() {
            stdout()
                .queue(Print("None"))?;
        }
        for result in results {
            stdout()
                .queue(Print(format!("{}", result.format("%Y %j %H"))))?;
        }

        stdout().flush()?;

        self.wait_for_char("\n\n\nPress any key to continue\n")?;

        Ok(())
    }
}