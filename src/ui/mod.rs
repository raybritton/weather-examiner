use crate::app::WeatherApp;
use std::io::stdout;
use crossterm::{ExecutableCommand, cursor};
use crossterm::style::Color;
use crate::Error;
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::MoveTo;
use crate::ui::ui_section::UiSection;
use crate::ui::utils::print_styled;
use crate::ui::main_menu::MainMenu;

mod weather_predictions;
mod ui_section;
mod utils;
mod weather_view;
mod main_menu;
mod import_data;
mod missing_records;

pub struct Ui {
    app: WeatherApp,
    reset_pos: Option<(u16, u16)>,
}

impl Ui {
    pub fn new(app: WeatherApp) -> Ui {
        return Ui {
            app,
            reset_pos: None,
        };
    }
}

impl Ui {
    pub fn run(&mut self) -> Result<(), Error> {
        stdout()
            .execute(cursor::Hide)?
            .execute(crossterm::terminal::SetSize(160, 40))?
            .execute(Clear(ClearType::All))?
            .execute(MoveTo(0, 0))?;

        print_styled("Weather prediction examiner\n\n", Color::Cyan, true)?;

        self.reset_pos = Some(crossterm::cursor::position()?);
        let pos = self.reset_pos.expect("No reset found when starting main menu");

        MainMenu::new(pos).run(&mut self.app)?;

        stdout()
            .execute(cursor::Show)?;

        Ok(())
    }
}