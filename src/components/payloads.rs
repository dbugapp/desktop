use crate::gui::Message;
use crate::storage::Storage;
use iced::widget::{button, column, container, scrollable, text};
use iced::{Element, Fill, Theme};
use serde_json::Value;

/// Creates a scrollable display of all received JSON payloads
pub fn payload_list<'a>(storage: &Storage, expanded_id: Option<&String>) -> Element<'a, Message> {
    let storage_rows = column(
        storage
            .get_all()
            .iter()
            .map(|(id, value)| {
                let is_expanded = expanded_id == Some(id);

                if is_expanded {
                    // Pretty print the JSON with proper indentation
                    let pretty_json = match serde_json::to_string_pretty(value) {
                        Ok(formatted) => formatted,
                        Err(_) => format!("{:?}", value),
                    };

                    // For expanded items, use a container with similar styling but not a button
                    container(column![text(pretty_json)].spacing(5).width(Fill))
                        .padding(10)
                        .width(Fill)
                        .style(|theme: &Theme| {
                            let palette = theme.extended_palette();
                            container::Style {
                                background: Some(palette.background.base.color.into()),
                                border: iced_core::border::rounded(5)
                                    .color(palette.secondary.base.color)
                                    .width(1.0),
                                ..container::Style::default()
                            }
                        })
                        .into()
                } else {
                    // For non-expanded items, use a button with secondary styling
                    let content = text(format_compact_json(value));

                    button(container(content).width(Fill))
                        .width(Fill)
                        .style(button::secondary)
                        .on_press(Message::TogglePayload(id.clone()))
                        .into()
                }
            })
            .collect::<Vec<_>>(),
    )
    .spacing(10)
    .padding(10);

    scrollable(container(storage_rows).padding(iced_core::Padding {
        right: 5.0,
        ..Default::default()
    }))
    .width(Fill)
    .height(Fill)
    .into()
}

/// Format JSON for compact display in collapsed view
fn format_compact_json(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            let preview_fields: Vec<String> = map
                .iter()
                .take(3) // Take first few fields for preview
                .map(|(k, v)| format!("{}: {}", k, format_brief_value(v)))
                .collect();

            let suffix = if map.len() > 3 { "..." } else { "" };
            format!("{{ {} {}}}", preview_fields.join(", "), suffix)
        }
        Value::Array(arr) => {
            let len = arr.len();
            if len == 0 {
                "[]".to_string()
            } else if len == 1 {
                format!("[{}]", format_brief_value(&arr[0]))
            } else {
                format!("[{} items]", len)
            }
        }
        _ => value.to_string(),
    }
}

/// Format a JSON value briefly for the preview
fn format_brief_value(value: &Value) -> String {
    match value {
        Value::String(s) => {
            if s.len() > 15 {
                format!("\"{}...\"", &s[0..12])
            } else {
                format!("\"{}\"", s)
            }
        }
        Value::Object(_) => "{...}".to_string(),
        Value::Array(arr) => format!("[{} items]", arr.len()),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
    }
}
