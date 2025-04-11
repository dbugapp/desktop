use crate::gui::Message;
use iced::widget::{column, row, text};
use iced::{Color, Element, Theme};

fn color_for_token(token: &str, is_key: bool, in_string: bool, theme: &Theme) -> Color {
    let palette = theme.extended_palette();
    if in_string {
        if is_key {
            palette.primary.base.color
        } else {
            palette.secondary.base.color
        }
    } else {
        match token {
            "{" | "}" => palette.background.strong.color,
            "[" | "]" => palette.background.strong.color,
            ":" => palette.secondary.base.color,
            "," => palette.background.strong.color,
            _ if token.chars().all(|c| c.is_numeric() || c == '-') => {
                palette.secondary.strong.color
            }
            _ => palette.primary.weak.color,
        }
    }
}

pub fn highlight_json(json: &str, theme: &Theme) -> Element<'static, Message> {
    let lines = json.lines().map(|line| line.to_owned()).collect::<Vec<_>>();

    column(
        lines
            .into_iter()
            .map(|line| {
                let mut is_key = true;
                let mut in_string = false;
                let mut current_token = String::new();
                let mut tokens = Vec::new();

                for c in line.chars() {
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

                row(tokens
                    .into_iter()
                    .map(|(token, is_key, in_string)| {
                        let color = color_for_token(&token, is_key, in_string, theme);
                        text(token)
                            .style(move |_| iced::widget::text::Style { color: Some(color) })
                            .into()
                    })
                    .collect::<Vec<Element<'_, Message>>>())
                .into()
            })
            .collect::<Vec<_>>(),
    )
    .spacing(2)
    .into()
}
