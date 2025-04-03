mod gui;
mod server;
mod settings;
mod storage;
mod theme;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    gui::gui()?;
    Ok(())
}
