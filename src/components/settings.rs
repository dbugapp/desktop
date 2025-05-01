use crate::app::Message;
use iced::widget::{column, container, radio, scrollable, text, row, horizontal_space, Row};
use iced::{Element, Fill, Theme, Length};
use crate::components::styles;

/// Creates the settings modal content with theme selection
pub fn settings_modal<'a>(current_theme: Theme) -> Element<'a, Message> {
    // Find the current theme index in Theme::ALL
    let current_index = Theme::ALL
        .iter()
        .position(|t| t.to_string() == current_theme.to_string())
        .unwrap_or(0);

    // Style for section headers
    let header_style = move |theme: &Theme| text::Style {
        color: theme.palette().text.into(),
    };
    let header_background_style = move |theme: &Theme| container::Style {
        background: Some(theme.extended_palette().background.weak.color.into()),
        ..container::Style::default()
    };

    // Shortcut rows
    let shortcut_row = |shortcut: &'a str, description: &'a str| -> Row<'a, Message> {
        row![
            text(shortcut).width(Length::Fixed(60.0)).size(12),
            horizontal_space().width(30.0),
            text(description).size(12),
        ]
        .spacing(10)
        .padding(5)
    };

    container(
        scrollable(
            column![
                // --- Shortcuts Section ---
                container(text("Shortcuts").size(16).style(header_style)).style(header_background_style).width(Fill).padding(5),
                column![
                    shortcut_row("Shift+Cmd+L", "Toggle visibility from anywhere"),
                    shortcut_row("Shift+Cmd+K", "Clear payloads from anywhere"),
                    shortcut_row("Cmd+,", "Open Settings"), // Added the Cmd+, shortcut
                ].spacing(5).padding(iced_core::Padding { top: 5.0, bottom: 15.0, right: 15.0, ..Default::default() }),

                // --- Theme Selection Section ---
                container(text("Customize Your Theme").size(16).style(header_style)).style(header_background_style).width(Fill).padding(5),
                container(
                    column(
                        Theme::ALL
                            .iter()
                            .enumerate()
                            .map(|(idx, theme)| {
                                container(
                                    radio(
                                        theme.to_string(),
                                        idx,
                                        Some(current_index),
                                        Message::ThemeChanged,
                                    ).text_size(12)
                                    .width(Fill)
                                    .style(|_, status| radio::Style {
                                        border_color: theme
                                            .extended_palette()
                                            .background
                                            .strong
                                            .color,
                                        text_color: theme.palette().text.into(),
                                        ..radio::default(theme, status)
                                    })
                                    .spacing(10),
                                )
                                .width(Fill)
                                .padding(10)
                                .style(move |_| container::Style {
                                    background: Some(
                                        theme.extended_palette().background.weak.color.into(),
                                    ),
                                    border: iced_core::border::rounded(5),
                                    ..container::Style::default()
                                })
                                .into()
                            })
                            .collect::<Vec<Element<Message>>>()
                    )
                    .spacing(10)
                )
                .padding(iced_core::Padding {
                    left: 5.0,
                    right: 5.0,
                    top: 10.0,
                    bottom: 5.0,
                })
            ]
            .padding(iced_core::Padding {
                left: 0.0,
                right: 5.0,
                top: 0.0,
                bottom: 5.0,
            }),
        )
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new().width(5).scroller_width(5),
            ))
    )
    .width(480)
    .height(480)
        .padding(iced_core::Padding {
            left: 0.0,
            right: 1.0,
            top: 5.0,
            bottom: 5.0,
        })
    .style(styles::container_modal)
    .into()
}
