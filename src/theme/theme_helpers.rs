use super::theme::Theme;
use iced::widget::{container, scrollable};
use iced::{Background, Color};

pub struct Subtle {}

impl Subtle {
    pub fn border(app_theme: &Theme) -> iced_core::Border {
        iced::Border {
            color: app_theme.border_accented,
            width: 1.0,
            radius: 10.into(),
        }
    }

    pub fn container(app_theme: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(app_theme.bg_elevated)),
            text_color: Some(app_theme.text),
            border: Self::border(app_theme),
            ..container::Style::default()
        }
    }

    pub fn scrollbar(app_theme: &Theme) -> scrollable::Style {
        let border = iced::Border {
            color: app_theme.border_accented,
            width: 1.0,
            radius: 5.into(),
        };

        let rail = scrollable::Rail {
            background: Some(app_theme.bg_elevated.into()),
            border,
            scroller: scrollable::Scroller {
                color: app_theme.bg_accented,
                border,
            },
        };

        scrollable::Style {
            container: container::Style {
                background: Some(Background::Color(app_theme.bg)),
                text_color: Some(app_theme.text),
                ..container::Style::default()
            },
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: None,
        }
    }
}
