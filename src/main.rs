mod server;
mod gui;
mod storage;
mod settings;
use tokio::sync::mpsc;

/// Main entry point of the application
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a channel for communication between server and GUI
    let (tx, rx) = mpsc::channel(100);

    // Run server in background task
    tokio::spawn(server::listen(tx));

    // Run GUI on main thread
    gui::gui(rx)?;

    Ok(())
}
