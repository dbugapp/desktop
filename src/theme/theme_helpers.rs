use super::theme::Theme;
use iced::widget::{container, scrollable};
use iced::{Background, Color};

pub struct Subtle {}

impl Subtle {
    pub fn border(app_theme: &Theme) -> impl Fn(&iced::Theme) -> iced_core::Border + '_ {
        move |_| iced::Border {
            color: app_theme.border_accented,
            width: 1.0,
            radius: 10.into(),
        }
    }

    pub fn container<'a>(app_theme: &'a Theme) -> impl Fn(&iced::Theme) -> container::Style + 'a {
        move |iced_theme| container::Style {
            background: Some(Background::Color(app_theme.bg_elevated)),
            text_color: Some(app_theme.text),
            border: Self::border(app_theme)(iced_theme),
            ..container::Style::default()
        }
    }

    pub fn scrollbar<'a>(
        app_theme: &'a Theme,
    ) -> impl Fn(&iced::Theme, scrollable::Status) -> scrollable::Style + 'a {
        move |iced_theme, _| {
            let create_rail = |app_theme: &Theme, iced_theme: &iced::Theme| {
                let border = Self::border(app_theme)(iced_theme);
                scrollable::Rail {
                    background: Some(app_theme.bg_elevated.into()),
                    border: border.clone(),
                    scroller: scrollable::Scroller {
                        color: app_theme.bg_accented.into(),
                        border,
                    },
                }
            };

            scrollable::Style {
                container: container::Style {
                    background: Some(Background::Color(app_theme.bg)),
                    text_color: Some(app_theme.text),
                    ..container::Style::default()
                },
                vertical_rail: create_rail(app_theme, iced_theme),
                horizontal_rail: create_rail(app_theme, iced_theme),
                gap: None,
            }
        }
    }
}
