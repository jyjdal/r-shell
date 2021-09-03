extern crate crossterm;
use crossterm::Command as CCommand;

use crate::BaseCommand;

use r_parser::data::CommandArg;

pub struct Clear {}

impl BaseCommand for Clear {
    fn name(&self) -> &str {
        "clear"
    }

    fn run(&self, _: Vec<CommandArg>) {
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
            .execute_winapi()
            .unwrap();
    }
}
