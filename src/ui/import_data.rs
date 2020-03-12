use crate::ui::ui_section::UiSection;
use crate::app::WeatherApp;
use crate::Error;
use std::io::stdout;
use crossterm::style::Print;
use crossterm::ExecutableCommand;

pub struct ImportData {
    reset_pos: (u16, u16)
}

impl ImportData {
    pub fn new(reset_pos: (u16, u16)) -> ImportData {
        return ImportData {
            reset_pos
        };
    }
}

impl UiSection for ImportData {
    fn run(&mut self, app: &mut WeatherApp) -> Result<(), Error> {
        self.reset(self.reset_pos)?;

        let dir = self.read_input("Enter a directory to import from\n")?;

        stdout().execute(Print("\nImporting\n"))?;

        app.import_data(dir)?;

        print_styled("\nDone", Color::Green, false);

        self.wait_for_char("\nPress any key to continue\n")?;

        Ok(())
    }
}