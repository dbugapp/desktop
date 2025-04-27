use crate::app::Message;
use iced::widget::{column, row, text, button, svg};
use iced::{Element, Theme};
use std::collections::{HashMap, HashSet};
use crate::components::styles;
use iced::widget::container;
use iced::Background;

/// Calculates the number of lines hidden within each collapsible block.
/// Returns a map where the key is the starting line index of a block
/// and the value is the count of lines contained within that block.
fn calculate_collapse_counts(lines: &[String]) -> HashMap<usize, usize> {
    let mut collapse_counts = HashMap::new();
    let mut indent_level: usize = 0;
    let mut block_starts: Vec<(usize, usize)> = Vec::new();

    for (idx, line_str) in lines.iter().enumerate() {
        let trimmed = line_str.trim();

        let starts_closing = trimmed.starts_with('}') || trimmed.starts_with(']');
        let ends_opening = trimmed.ends_with('{') || trimmed.ends_with('[');

        let indent_before_line = indent_level;

        if starts_closing {
            indent_level = indent_level.saturating_sub(1);
            if let Some((_start_idx, start_indent)) = block_starts.last() {
                if indent_level == *start_indent {
                    let popped_start_idx = block_starts.pop().unwrap().0;
                    let line_count = idx.saturating_sub(popped_start_idx).saturating_sub(1);
                    collapse_counts.insert(popped_start_idx, line_count);
                }
            }
        }

        if ends_opening && idx != 0 {
            block_starts.push((idx, indent_before_line));
        }

        if ends_opening {
            indent_level += 1;
        }
    }
    collapse_counts
}

pub fn highlight_json(
    json: &str,
    theme: &Theme,
    collapsed_lines: &HashSet<usize>,
    search_query: &str,
) -> Element<'static, Message> {
    let lines = json.lines().map(|line| line.to_owned()).collect::<Vec<_>>();
    let collapse_counts = calculate_collapse_counts(&lines);
    let palette = theme.extended_palette();
    let background_strong_color = palette.background.strong.color;
    let secondary_base_text_color = palette.secondary.base.text;
    let primary_strong_color = palette.primary.strong.color;
    let background_weak_color = palette.background.weak.color;
    let secondary_base_color = palette.secondary.base.color;
    let success_weak_color = palette.success.weak.color;
    let primary_weak_color = palette.primary.weak.color;
    let search_highlight_color = palette.background.weak.color;
    let search_text_color = palette.success.strong.color;

    let mut elements = Vec::new();
    let mut indent_level: usize = 0;
    let indent_size = 2;
    let mut skip_depth: Option<usize> = None;

    for (idx, line) in lines.into_iter().enumerate() {
        let trimmed_line = line.trim();

        if let Some(depth) = skip_depth {
            if indent_level > depth {
                if trimmed_line.starts_with('}') || trimmed_line.starts_with(']') {
                    indent_level = indent_level.saturating_sub(1);
                }
                if trimmed_line.ends_with('{') || trimmed_line.ends_with('[') {
                    indent_level += 1;
                }
                continue;
            } else {
                skip_depth = None;
            }
        }

        let current_indent = indent_level;
        if trimmed_line.starts_with('}') || trimmed_line.starts_with(']') {
            indent_level = indent_level.saturating_sub(1);
        }

        let is_match = !search_query.is_empty() && line.to_lowercase().contains(&search_query.to_lowercase());

        let is_collapsible = (trimmed_line.ends_with('{') || trimmed_line.ends_with('[')) && idx != 0;
        let is_collapsed = collapsed_lines.contains(&idx);

        let mut is_key = true;
        let mut in_string = false;
        let mut current_token = String::new();
        let mut tokens = Vec::new();
        let mut prev_char = '\0'; // Track previous char for escapes

        if !(is_collapsed && is_collapsible && trimmed_line.is_empty()) {
            for c in trimmed_line.chars() {
                if c == '"' {
                    if in_string && prev_char != '\\' {
                        // End of string (unescaped quote)
                        tokens.push((current_token.clone(), is_key, true));
                        current_token.clear();
                        in_string = false;
                        // Delimiter logic below will set is_key if appropriate
                    } else if !in_string {
                        // Start of string
                        // Push any pending non-string token before starting the string
                        if !current_token.is_empty() { // Simplified check
                            tokens.push((current_token.clone(), is_key, false));
                            current_token.clear();
                        }
                        in_string = true;
                        // Don't add the quote to current_token
                    } else {
                        // Inside string, and quote char itself
                        if prev_char == '\\' {
                            // It's an escaped quote: remove the preceding \ and add "
                            current_token.pop(); // Remove the already added \
                            current_token.push(c); // Add the "
                        } else {
                             // An unescaped quote char within a string? Technically invalid JSON,
                             // but we'll treat it like a regular char.
                             current_token.push(c);
                        }
                    }
                } else if in_string {
                    // Regular character inside string
                    current_token.push(c);
                } else {
                    // Character outside string
                    // Push any accumulated non-string token before handling the delimiter/char
                    if !current_token.is_empty() { // Simplified check
                        tokens.push((current_token.clone(), is_key, false));
                        current_token.clear();
                    }
                    // Handle the current delimiter or start of a new non-string token
                    if ['{', '}', '[', ']', ':', ','].contains(&c) {
                        tokens.push((c.to_string(), false, false));
                        if c == ',' || c == '{' || c == '[' {
                            is_key = true; // Next token *could* be a key
                        } else if c == ':'{
                             is_key = false; // Next token must be a value
                        }
                    } else if !c.is_whitespace() {
                        // Start accumulating a non-string token (number, bool, null)
                        current_token.push(c);
                    }
                    // Ignore whitespace outside strings
                }
                prev_char = c; // Update prev_char for the next iteration
            }
            // After the loop, push any remaining token
            if !current_token.is_empty() { // Simplified check
                tokens.push((current_token, is_key, false));
            }
        }

        let row_element = row(
            tokens
                .into_iter()
                .flat_map(|(token, is_key, in_string)| { // Use flat_map to handle multiple elements per token
                    // Determine original syntax color
                    let original_color = if in_string {
                        if is_key {
                            secondary_base_text_color
                        } else {
                            primary_strong_color
                        }
                    } else {
                        match token.as_str() {
                            "{" | "}" => background_weak_color,
                            "[" | "]" => background_weak_color,
                            ":" => secondary_base_color,
                            "," => background_strong_color,
                            _ if token.trim().parse::<f64>().is_ok() => {
                                success_weak_color
                            }
                            _ => primary_weak_color,
                        }
                    };

                    let mut token_elements = Vec::new();

                    // Check for search match (case-insensitive)
                    if !search_query.is_empty() {
                        if let Some(match_start_lowercase) = token.to_lowercase().find(&search_query.to_lowercase()) {
                            let match_end_lowercase = match_start_lowercase + search_query.len();

                            // Extract parts based on original token casing and convert to owned Strings
                            let before = token[..match_start_lowercase].to_string();
                            let matched = token[match_start_lowercase..match_end_lowercase].to_string();
                            let after = token[match_end_lowercase..].to_string();

                            if !before.is_empty() {
                                token_elements.push(
                                    text(before) // Pass owned String
                                        .style(move |_| iced::widget::text::Style { color: Some(original_color) })
                                        .into()
                                );
                            }
                            token_elements.push(
                                text(matched) // Pass owned String
                                    // Apply search text color to the matched part
                                    .style(move |_| iced::widget::text::Style { color: Some(search_text_color) })
                                    .into()
                            );
                            if !after.is_empty() {
                                token_elements.push(
                                    text(after) // Pass owned String
                                        .style(move |_| iced::widget::text::Style { color: Some(original_color) })
                                        .into()
                                );
                            }
                        } else {
                            // No match in this token, pass owned String
                            token_elements.push(
                                text(token.to_string()) // Pass owned String
                                    .style(move |_| iced::widget::text::Style { color: Some(original_color) })
                                    .into()
                            );
                        }
                    } else {
                        // Search query is empty, pass owned String
                        token_elements.push(
                            text(token.to_string()) // Pass owned String
                                .style(move |_| iced::widget::text::Style { color: Some(original_color) })
                                .into()
                        );
                    }
                    token_elements // Return Vec<Element> for flat_map
                })
                .collect::<Vec<Element<'_, Message>>>(), // Collect the flattened elements
        );

        let collapse_button_icon = if is_collapsible {
            if is_collapsed {
                include_bytes!("../../assets/icons/mdi--caret-up.svg").as_slice()
            } else {
                include_bytes!("../../assets/icons/mdi--caret-down.svg").as_slice()
            }
        } else {
            &[]
        };

        let collapse_element: Element<'_, Message> = if is_collapsible {
            button(
                svg(svg::Handle::from_memory(collapse_button_icon))
                    .style(styles::svg_style_secondary),
            )
            .width(15)
            .padding(0)
            .style(button::secondary)
            .on_press(Message::ToggleJsonSection(idx))
            .into()
        } else {
            text(" ").width(15).into()
        };

        let indented_row = row![
            collapse_element,
            text(format!("{:>3} ", idx + 1))
                .style(move |_theme: &Theme| iced::widget::text::Style {
                    color: Some(background_strong_color),
                })
                .width(30),
            text(" ".repeat(current_indent * indent_size)),
            if is_collapsible && is_collapsed {
                let count = collapse_counts.get(&idx).copied().unwrap_or(0);
                let closing_char = if trimmed_line.ends_with('{') { "}" } else { "]" };
                let token_color = match closing_char {
                    "}" | "]" => background_weak_color,
                     _ => primary_weak_color,
                };
                let count_color = background_strong_color;

                let count_indicator = row![
                    text(format!(" {count} lines "))
                        .style(move |_| iced::widget::text::Style { color: Some(count_color) }),
                    text(closing_char)
                        .style(move |_| iced::widget::text::Style { color: Some(token_color) })
                ];

                row![row_element, count_indicator]
            } else {
                row_element
            }
        ];

        let row_container = container(indented_row);
        let styled_row = if is_match {
            row_container.style(move |_: &Theme| container::Style {
                background: Some(Background::Color(search_highlight_color)),
                ..Default::default()
            })
        } else {
            row_container.style(container::transparent)
        };

        elements.push(styled_row.into());

        if is_collapsible && is_collapsed {
            skip_depth = Some(current_indent);
            if trimmed_line.ends_with('{') || trimmed_line.ends_with('[') {
                indent_level += 1;
            }
        } else if trimmed_line.ends_with('{') || trimmed_line.ends_with('[') {
            indent_level += 1;
        }
    }

    column(elements).spacing(2).into()
}

