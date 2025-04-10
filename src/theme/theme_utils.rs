use iced::Color;
use palette::{convert::IntoColor, rgb::Rgb, Clamp, FromColor, Oklch, Srgb};

pub fn oklch(l: f32, c: f32, h_degrees: f32) -> Color {
    // Create an Oklch color from the given components
    let oklch_color = Oklch::new(l, c, h_degrees.to_radians());

    // Convert to sRGB directly - palette handles the necessary conversions internally
    let srgb: Srgb = Srgb::from_color(oklch_color);

    // Clamp and extract components
    let srgb_clamped = srgb.clamp();

    Color::from_rgb(
        srgb_clamped.red as f32,
        srgb_clamped.green as f32,
        srgb_clamped.blue as f32,
    )
}

pub fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');

    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);

        Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    } else {
        Color::BLACK
    }
}
