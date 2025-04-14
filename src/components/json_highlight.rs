use crate::components::custom_highlighter::{Highlight, Highlighter, Settings};
use crate::gui::Message;
use iced::widget::{text_editor, Column};
use iced::{Element, Theme};

pub fn highlight_json<'a>(
    content: &'a text_editor::Content,
    _theme: &Theme,
) -> Element<'a, Message> {
    Column::new()
        .push(text_editor(content).highlight_with::<Highlighter>(
            Settings::new(vec![], Highlight::default_style, "json"),
            Highlight::to_format,
        ))
        .into()
}
