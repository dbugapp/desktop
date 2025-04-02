mod gui;
mod storage;
mod settings;
mod server;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    gui::gui()?;
    Ok(())
}
