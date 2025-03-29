mod gui;
mod server;
mod storage;
mod highlighter;

use iced::Sandbox;

fn main() -> iced::Result {
    // Initialize the server
    server::start();

    // Run the GUI application
    gui::App::run(iced::Settings::default())
}
