use crate::gui::Message;
use iced::widget::{column, row, text};
use iced::{Color, Element};
use serde_json::Value;

fn color_for_json(value: &Value) -> Color {
    match value {
        Value::String(_) => Color::from_rgb(0.8, 0.6, 0.2),
        Value::Number(_) => Color::from_rgb(0.2, 0.6, 0.8),
        Value::Bool(_) => Color::from_rgb(0.8, 0.3, 0.3),
        Value::Null => Color::from_rgb(0.5, 0.5, 0.5),
        _ => Color::WHITE,
    }
}

fn render_json(value: &Value) -> Element<'static, Message> {
    match value {
        Value::Object(map) => column(
            map.iter()
                .map(|(k, v)| {
                    let key = k.clone();
                    row![
                        text(format!("\"{}\": ", key)).style(move |_| iced::widget::text::Style {
                            color: Some(Color::from_rgb(0.8, 0.6, 0.2))
                        }),
                        render_json(v)
                    ]
                    .spacing(5)
                    .into()
                })
                .collect::<Vec<_>>(),
        )
        .spacing(5)
        .into(),
        Value::Array(arr) => column(arr.iter().map(render_json).collect::<Vec<_>>())
            .spacing(5)
            .into(),
        _ => {
            let val_str = value.to_string();
            let color = color_for_json(value);
            text(val_str)
                .style(move |_| iced::widget::text::Style { color: Some(color) })
                .into()
        }
    }
}

pub fn highlight_json(json: &str) -> Element<'static, Message> {
    let parsed_json: Value = serde_json::from_str(json).unwrap_or(Value::Null);
    render_json(&parsed_json)
}
