use crate::app::Message;
use crate::components::json_highlight::highlight_json;
use crate::components::styles;
use chrono::{DateTime, Utc};
use core::time::Duration;
use iced::widget::{button, column, container, row, scrollable, stack, svg, text, text_input};
use iced::{Element, Fill, Theme};
use millisecond::prelude::*;
use serde_json::Value;
use std::collections::HashSet;

/// Converts a timestamp ID into a human-readable relative time string
fn human_readable_time(id: &str) -> String {
    id.parse::<i64>()
        .ok()
        .and_then(DateTime::<Utc>::from_timestamp_millis)
        .map(|time| Utc::now().signed_duration_since(time))
        .map_or_else(
            || "Invalid timestamp".to_string(),
            |duration| Duration::from_millis(duration.num_milliseconds() as u64).relative(),
        )
}

/// Creates a scrollable display of received JSON payloads using cached data
pub fn payload_list<'a>(
    payloads: &'a [(String, Value)],
    expanded_id: Option<&String>,
    theme: &Theme,
    collapsed_json_lines: &HashSet<usize>,
    max_payload_height: f32,
    search_query: &str,
) -> Element<'a, Message> {
    let storage_rows = column(
        payloads
            .iter()
            .map(|(id, value)| {
                let is_expanded = expanded_id == Some(id);
                let timestamp = human_readable_time(id);

                if is_expanded {
                    let pretty_json = serde_json::to_string_pretty(value).unwrap_or_else(|err| {
                        eprintln!("Error prettifying payload {id}: {err}");
                        format!("{{ \"error\": \"Failed to render JSON: {err}\" }}")
                    });

                    let highlighted_json =
                        highlight_json(&pretty_json, theme, collapsed_json_lines, search_query);

                    let close_svg = svg(svg::Handle::from_memory(
                        include_bytes!("../../assets/icons/mdi--caret-down.svg").as_slice(),
                    ))
                        .width(Fill)
                        .height(Fill)
                        .style(styles::svg_style_secondary);

                    let delete_svg = svg(svg::Handle::from_memory(
                        include_bytes!("../../assets/icons/mdi--trash-can.svg").as_slice(),
                    ))
                        .width(Fill)
                        .height(Fill)
                        .style(styles::svg_style_primary);

                    container(
                        stack![
                            container(
                                scrollable(container(highlighted_json).padding(10))
                                    .direction(scrollable::Direction::Vertical(
                                        scrollable::Scrollbar::new().width(3).scroller_width(3),
                                    ))
                                    .width(Fill)
                            ).padding(
                            iced_core::Padding {
                                    top: 3.0,
                                    right: 2.0,
                                    bottom: 3.0,
                                    ..Default::default()
                            })
                            .max_height(max_payload_height),
                            container(
                                row![
                                    container(text(timestamp).size(10.0))
                                        .padding(3.0)
                                        .align_x(iced::alignment::Horizontal::Right)
                                        .align_y(iced::alignment::Vertical::Bottom)
                                        .width(Fill),
                                    text_input("Search...", search_query).width(150.0)
                                        .on_input(Message::SearchQueryChanged)
                                        .size(12)
                                        .padding(2),
                                    button(delete_svg)
                                        .style(button::danger)
                                        .width(18)
                                        .height(18)
                                        .padding(1)
                                        .on_press(Message::DeletePayload(id.clone())),
                                    button(close_svg)
                                        .width(18)
                                        .height(18)
                                        .padding(0)
                                        .on_press(Message::TogglePayload(id.clone()))
                                ]
                                .spacing(5)
                            )
                            .padding(10)
                            .align_top(Fill)
                            .align_right(Fill)
                            .width(Fill),
                        ]
                            .width(Fill),
                    )
                        .width(Fill)
                        .style(styles::container_code)
                        .into()
                } else {
                    let expand_svg = svg(svg::Handle::from_memory(
                        include_bytes!("../../assets/icons/mdi--caret-up.svg").as_slice(),
                    ))
                        .width(Fill)
                        .height(Fill)
                        .style(styles::svg_style_secondary);

                    let delete_svg = svg(svg::Handle::from_memory(
                        include_bytes!("../../assets/icons/mdi--trash-can.svg").as_slice(),
                    ))
                        .width(Fill)
                        .height(Fill)
                        .style(styles::svg_style_primary);

                    button(
                        container(
                            row![
                                container(text(format!("{value}")).size(14).height(18.0))
                                    .width(Fill),
                                container(text(timestamp).size(10.0))
                                    .padding(4.0)
                                    .align_x(iced::alignment::Horizontal::Right)
                                    .align_y(iced::alignment::Vertical::Center),
                                button(delete_svg)
                                    .style(button::danger)
                                    .width(18)
                                    .height(18)
                                    .padding(1)
                                    .on_press(Message::DeletePayload(id.clone())),
                                button(expand_svg).width(18).height(18).padding(0)
                            ]
                                .spacing(5),
                        )
                            .padding(10)
                            .width(Fill)
                            .style(styles::container_code_closed),
                    )
                        .style(button::text)
                        .width(Fill)
                        .on_press(Message::TogglePayload(id.clone()))
                        .padding(0)
                        .into()
                }
            })
            .collect::<Vec<_>>(),
    )
        .spacing(10)
        .padding(iced_core::Padding {
            right: 5.0,
            left: 5.0,
            top: 1.0,
            bottom: 5.0,
        });

    scrollable(container(storage_rows).padding(iced_core::Padding {
        right: 5.0,
        ..Default::default()
    }))
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::new().width(5).scroller_width(5),
        ))
        .id(iced::widget::scrollable::Id::new("payload_scroll"))
        .width(Fill)
        .height(Fill)
        .into()
}
