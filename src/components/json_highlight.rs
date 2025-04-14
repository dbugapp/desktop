use crate::gui::Message;
use iced::widget::{column, row, text as iced_text};
use iced::{Color, Element, Theme};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;

pub fn highlight_json(json: &str, theme: &Theme) -> Element<'static, Message> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("json").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    let lines = json
        .lines()
        .map(|line| {
            let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
            let row_element = row(ranges
                .into_iter()
                .map(|(style, text)| {
                    let color = Color::from_rgba8(
                        style.foreground.r,
                        style.foreground.g,
                        style.foreground.b,
                        (style.foreground.a as f32 / 255.0)
                            * theme.extended_palette().background.base.color.a,
                    );
                    iced_text(text.to_owned())
                        .style(move |_| iced::widget::text::Style { color: Some(color) })
                        .into()
                })
                .collect::<Vec<Element<'_, Message>>>());

            row_element.into()
        })
        .collect::<Vec<_>>();

    column(lines).spacing(2).into()
}
