use super::theme::Theme;
use iced::widget::{button, container, scrollable, Scrollable};
use iced::Shadow;
use iced::{Background, Color};

pub struct ButtonStyle {
    pub background: Option<Background>,
    pub text_color: Color,
}

pub struct Subtle {
    pub background: Option<Background>,
}

impl ButtonStyle {
    pub fn from_theme(theme: &Theme) -> button::Style {
        button::Style {
            background: Some(Background::Color(theme.primary)),
            text_color: theme.text,
            ..Default::default()
        }
    }
}

impl Subtle {
    pub fn border(app_theme: &Theme) -> impl Fn(&iced::Theme) -> iced_core::Border + '_ {
        move |_iced_theme| iced::Border {
            color: app_theme.border_accented,
            width: 1.0,
            radius: 10.into(),
        }
    }
    pub fn container<'a>(app_theme: &'a Theme) -> impl Fn(&iced::Theme) -> container::Style + 'a {
        move |iced_theme| container::Style {
            background: Some(Background::Color(app_theme.bg_elevated)),
            text_color: Some(app_theme.text),
            border: Subtle::border(app_theme)(iced_theme), // Call the function to get the actual Border
            ..container::Style::default()
        }
    }

    pub fn scrollbar<'a>(
        app_theme: &'a Theme,
    ) -> impl Fn(&iced::Theme, scrollable::Status) -> scrollable::Style + 'a {
        move |iced_theme, _status| {
            // Get the border by calling the border function
            let border = Subtle::border(app_theme)(iced_theme);

            // Use the border for the scroller
            let scroller = scrollable::Scroller {
                color: app_theme.bg_accented.into(),
                border,
            };

            // Use the border for the rail
            let rail = scrollable::Rail {
                background: Some(app_theme.bg_elevated.into()),
                border,
                scroller,
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
}
