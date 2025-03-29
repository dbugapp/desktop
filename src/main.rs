mod server;
mod gui;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run server in background task
    tokio::spawn(server::listen());

    // Run GUI on main thread
    gui::gui()?;

    Ok(())
}