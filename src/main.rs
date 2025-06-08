mod message;
mod state;
mod ui;

use iced::Application;
use state::Dashboard;

pub fn main() -> iced::Result {
    Dashboard::run(Default::default())
}
