mod gui;
mod driver_db;
mod hardware;
mod installer;

use gui::App;

fn main() -> iced::Result {
    App::run(iced::Settings::default())
}