use crate::gui::Message;
use iced::widget::{column, row, text};
use iced::{Color, Element, Theme};

fn color_for_token(token: &str, is_key: bool, in_string: bool, theme: &Theme) -> Color {
    let palette = theme.extended_palette();
    if in_string {
        if is_key {
            palette.primary.base.color // Key color
        } else {
            palette.primary.strong.color // String value color
        }
    } else {
        match token {
            "{" | "}" => palette.background.weak.color, // Curly braces color
            "[" | "]" => palette.background.weak.color, // Brackets color
            ":" => palette.secondary.base.color,        // Colon color
            "," => palette.background.strong.color,     // Comma color
            _ if token.trim().parse::<f64>().is_ok() => {
                palette.success.weak.color // Numeric value color (integers, decimals, scientific notation)
            }
            _ => palette.primary.weak.color, // Default color
        }
    }
}

pub fn highlight_json(json: &str, theme: &Theme) -> Element<'static, Message> {
    let lines = json.lines().map(|line| line.to_owned()).collect::<Vec<_>>();

    let mut indent_level: usize = 0;
    let indent_size = 2;

    column(
        lines
            .into_iter()
            .map(|line| {
                let trimmed_line = line.trim();
                if trimmed_line.starts_with('}') || trimmed_line.starts_with(']') {
                    indent_level = indent_level.saturating_sub(1);
                }

                let mut is_key = true;
                let mut in_string = false;
                let mut current_token = String::new();
                let mut tokens = Vec::new();

                for c in trimmed_line.chars() {
                    if c == '"' {
                        current_token.push(c);
                        if in_string {
                            tokens.push((current_token.clone(), is_key, true));
                            current_token.clear();
                            if is_key {
                                is_key = false;
                            }
                        }
                        in_string = !in_string;
                    } else if in_string {
                        current_token.push(c);
                    } else if ['{', '}', '[', ']', ':', ','].contains(&c) {
                        if !current_token.trim().is_empty() {
                            tokens.push((current_token.clone(), is_key, false));
                            current_token.clear();
                        }
                        tokens.push((c.to_string(), false, false));
                        if c == ',' || c == '{' || c == '[' {
                            is_key = true;
                        }
                    } else {
                        current_token.push(c);
                    }
                }
                if !current_token.trim().is_empty() {
                    tokens.push((current_token, is_key, false));
                }

                let row_element = row(tokens
                    .into_iter()
                    .map(|(token, is_key, in_string)| {
                        let color = color_for_token(&token, is_key, in_string, theme);
                        text(token)
                            .style(move |_| iced::widget::text::Style { color: Some(color) })
                            .into()
                    })
                    .collect::<Vec<Element<'_, Message>>>());

                let indented_row = row![text(" ".repeat(indent_level * indent_size)), row_element];

                if trimmed_line.ends_with('{') || trimmed_line.ends_with('[') {
                    indent_level += 1;
                }

                indented_row.into()
            })
            .collect::<Vec<_>>(),
    )
    .spacing(2)
    .into()
}
