mod message;
mod state;
mod ui;
mod config;

use iced::Application;
use state::Dashboard;

pub fn main() -> iced::Result {
    Dashboard::run(Default::default())
}
