use crate::app::Message;
use iced::widget::{column, row, text, button, svg};
use iced::{Element, Theme, Center};
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
    let line_number_color = palette.background.strong.color;
    let key_color = palette.secondary.base.text;
    let string_value_color = palette.primary.strong.color;
    let bracket_color = palette.background.weak.color;
    let colon_color = palette.secondary.base.color;
    let number_color = palette.success.weak.color;
    let other_value_color = palette.primary.weak.color;
    let comma_color = palette.background.strong.color;
    let search_highlight_bg = palette.background.weak.color;
    let search_match_text_color = palette.success.strong.color;

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
                            key_color
                        } else {
                            string_value_color
                        }
                    } else {
                        match token.as_str() {
                            "{" | "}" => bracket_color,
                            "[" | "]" => bracket_color,
                            ":" => colon_color,
                            "," => comma_color,
                            _ if token.trim().parse::<f64>().is_ok() => {
                                number_color
                            }
                            _ => other_value_color,
                        }
                    };

                    let mut token_elements = Vec::new();

                    // Function to create a text element, escaping newlines
                    let create_text_element = |content: String, color: iced::Color| -> Element<'static, Message> {
                        text(content.replace('\n', "\\n")) // Replace \n with \\n before creating text widget
                            .style(move |_| iced::widget::text::Style { color: Some(color) })
                            .into()
                    };

                    // Check for search match (case-insensitive) and handle multiple matches
                    if !search_query.is_empty() {
                        let mut current_idx = 0;
                        let token_lowercase = token.to_lowercase();
                        let query_lowercase = search_query.to_lowercase();

                        // Use match_indices for case-insensitive search simulation
                        for (match_start_idx, matched_part_lowercase) in token_lowercase.match_indices(&query_lowercase) {
                            let match_end_idx = match_start_idx + matched_part_lowercase.len();

                            // Add non-matching part before the current match
                            if match_start_idx > current_idx {
                                let non_matching_part = token[current_idx..match_start_idx].to_string();
                                token_elements.push(create_text_element(non_matching_part, original_color));
                            }

                            // Add the matching part
                            let matching_part = token[match_start_idx..match_end_idx].to_string();
                            token_elements.push(create_text_element(matching_part, search_match_text_color));

                            current_idx = match_end_idx;
                        }

                        // Add any remaining non-matching part after the last match
                        if current_idx < token.len() {
                            let remaining_part = token[current_idx..].to_string();
                            token_elements.push(create_text_element(remaining_part, original_color));
                        }

                        // If no matches were found at all in this token
                        if token_elements.is_empty() {
                             token_elements.push(create_text_element(token.to_string(), original_color));
                        }
                    } else {
                        // Search query is empty, render the whole token normally
                        token_elements.push(create_text_element(token.to_string(), original_color));
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
                    color: Some(line_number_color),
                })
            .size(11)
                .width(30),
            text(" ".repeat(current_indent * indent_size)),
            if is_collapsible && is_collapsed {
                let count = collapse_counts.get(&idx).copied().unwrap_or(0);
                let closing_char = if trimmed_line.ends_with('{') { "}" } else { "]" };
                let token_color = match closing_char {
                    "}" | "]" => bracket_color,
                     _ => other_value_color,
                };
                let count_color = line_number_color;

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
        ].align_y(Center);

        let row_container = container(indented_row);
        let styled_row = if is_match {
            row_container.style(move |_: &Theme| container::Style {
                background: Some(Background::Color(search_highlight_bg)),
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

