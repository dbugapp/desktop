use crate::gui::Message;
use iced::widget::{column, row, text};
use iced::{Color, Element, Theme};

fn color_for_token(token: &str, theme: &Theme) -> Color {
    let palette = theme.extended_palette();
    match token.chars().next() {
        Some('"') => palette.primary.base.color,
        Some('{') | Some('}') | Some('[') | Some(']') => palette.background.weak.color,
        Some(':') => palette.secondary.base.color,
        Some('0'..='9') | Some('-') => palette.secondary.strong.color,
        _ => palette.primary.weak.color,
    }
}

pub fn highlight_json(json: &str, theme: &Theme) -> Element<'static, Message> {
    let lines = json.lines().map(|line| line.to_owned()).collect::<Vec<_>>();

    column(
        lines
            .into_iter()
            .map(|line| {
                let tokens = line
                    .split_inclusive(|c: char| c.is_whitespace() || ['{', '}', '[', ']', ':', ','].contains(&c))
                    .map(|token| token.to_owned())
                    .collect::<Vec<_>>();

                row(
                    tokens
                        .into_iter()
                        .map(|token| {
                            let color = color_for_token(token.trim(), theme);
                            text(token)
                                .style(move |_| iced::widget::text::Style { color: Some(color) })
                                .into()
                        })
                        .collect::<Vec<Element<'_, Message>>>(),
                )
                .into()
            })
            .collect::<Vec<_>>(),
    )
    .spacing(2)
    .into()
}
