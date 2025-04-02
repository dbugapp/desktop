mod gui;
mod storage;
mod settings;
mod server;

/// Main entry point of the application
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run server in background task

    // Run GUI on main thread
    gui::gui()?;

    Ok(())
}
