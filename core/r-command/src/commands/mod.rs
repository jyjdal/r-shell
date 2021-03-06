mod cat;
mod cd;
mod clear;
mod exit;
mod ls;
mod args;

pub use cat::Cat;
pub use cd::Cd;
pub use clear::Clear;
pub use exit::Exit;
pub use ls::Ls;

use crate::BaseCommand;

pub fn all_commands() -> Vec<Box<dyn BaseCommand>> {
    vec![
        Box::new(Clear {}),
        Box::new(Ls {}),
        Box::new(Exit {}),
        Box::new(Cd {}),
        Box::new(Cat {}),
    ]
}
