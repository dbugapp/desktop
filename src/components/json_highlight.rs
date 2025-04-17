use crate::gui::Message;
use iced::widget::{column, row, text, button, svg};
use iced::{Color, Element, Theme};
use std::collections::{HashMap, HashSet};
use crate::components::styles;

fn color_for_token(token: &str, is_key: bool, in_string: bool, theme: &Theme) -> Color {
    let palette = theme.extended_palette();
    if in_string {
        if is_key {
            palette.secondary.base.text // Key color
        } else {
            palette.primary.strong.color // String value color
        }
    } else {
        match token {
            "{" | "}" => palette.background.weak.color, // Curly braces color
            "[" | "]" => palette.background.weak.color, // Brackets color
            ":" => palette.secondary.base.color,        // Colon color
            "," => palette.background.strong.color,     // Comma color
            _ if token.trim().parse::<f64>().is_ok() => {
                palette.success.weak.color // Numeric value color (integers, decimals, scientific notation)
            }
            _ => palette.primary.weak.color, // Default color
        }
    }
}

/// Calculates the number of lines hidden within each collapsible block.
/// Returns a map where the key is the starting line index of a block
/// and the value is the count of lines contained within that block.
fn calculate_collapse_counts(lines: &[String]) -> HashMap<usize, usize> {
    let mut collapse_counts = HashMap::new();
    let mut indent_level: usize = 0;
    // Stores the starting line index and the indentation level at that point
    let mut block_starts: Vec<(usize, usize)> = Vec::new();

    for idx in 0..lines.len() {
        let line_str = &lines[idx];
        let trimmed = line_str.trim();

        // Check if the line starts with a closing character or ends with an opening one
        let starts_closing = trimmed.starts_with('}') || trimmed.starts_with(']');
        let ends_opening = trimmed.ends_with('{') || trimmed.ends_with('[');

        // Store the indent level *before* potentially adjusting it for the current line
        let indent_before_line = indent_level;

        // If the line starts with a closing character, decrease indent level
        if starts_closing {
            indent_level = indent_level.saturating_sub(1);
            // Check if this closing character matches the indent level of the last opened block
            if let Some((_start_idx, start_indent)) = block_starts.last() {
                if indent_level == *start_indent {
                    // Matched the block, pop it from the stack
                    let popped_start_idx = block_starts.pop().unwrap().0;
                    // Calculate lines *between* start (exclusive) and end (exclusive)
                    let line_count = idx.saturating_sub(popped_start_idx).saturating_sub(1);
                    // Store the count, keyed by the starting line index
                    collapse_counts.insert(popped_start_idx, line_count);
                }
            }
        }

        // If the line ends with an opening character (and it's not the first line),
        // record it as a potential start of a collapsible block.
        // Use the indent level *before* this line was processed.
        if ends_opening && idx != 0 {
            block_starts.push((idx, indent_before_line));
        }

        // If the line ends with an opening character, increase indent level for the next line
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
) -> Element<'static, Message> {
    let lines = json.lines().map(|line| line.to_owned()).collect::<Vec<_>>();
    // Pre-calculate the number of lines within each collapsible block
    let collapse_counts = calculate_collapse_counts(&lines);

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
                continue; // Skip rendering this line
            } else {
                skip_depth = None; // We've exited the collapsed block
            }
        }

        let current_indent = indent_level;
        if trimmed_line.starts_with('}') || trimmed_line.starts_with(']') {
            indent_level = indent_level.saturating_sub(1);
        }

        // change this to check if trimmed_line ends with { or [
        let is_collapsible = (trimmed_line.ends_with('{') || trimmed_line.ends_with('[')) && idx != 0;
        let is_collapsed = collapsed_lines.contains(&idx);

        let mut is_key = true;
        let mut in_string = false;
        let mut current_token = String::new();
        let mut tokens = Vec::new();

        // Tokenize the line (only if not collapsed and needs rendering)
        if !(is_collapsed && is_collapsible) || !trimmed_line.is_empty() {
            for c in trimmed_line.chars() {
                if c == '"' {
                    if in_string {
                        if current_token.starts_with('"') && current_token.ends_with('"') {
                            current_token =
                                current_token[1..current_token.len() - 1].to_string();
                        }
                        tokens.push((current_token.clone(), is_key, true));
                        current_token.clear();
                        if is_key {
                            is_key = false;
                        }
                    }
                    in_string = !in_string;
                } else if in_string {
                    current_token.push(c);
                } else if ['{', '}', '[', ']', ':', ','].contains(&c) {
                    if !current_token.trim().is_empty() {
                        tokens.push((current_token.clone(), is_key, false));
                        current_token.clear();
                    }
                    tokens.push((c.to_string(), false, false));
                    if c == ',' || c == '{' || c == '[' {
                        is_key = true;
                    }
                } else {
                    current_token.push(c);
                }
            }
            if !current_token.trim().is_empty() {
                tokens.push((current_token, is_key, false));
            }
        }

        let row_element = row(
            tokens
                .into_iter()
                .map(|(token, is_key, in_string)| {
                    let color = color_for_token(&token, is_key, in_string, theme);
                    text(token)
                        .style(move |_| iced::widget::text::Style { color: Some(color) })
                        .into()
                })
                .collect::<Vec<Element<'_, Message>>>(),
        );

        let collapse_button_icon = if is_collapsible {
            if is_collapsed {
                include_bytes!("../../assets/icons/mdi--caret-up.svg").as_slice()
            } else {
                include_bytes!("../../assets/icons/mdi--caret-down.svg").as_slice()
            }
        } else {
            // Provide a default empty slice if not collapsible to satisfy the type checker
            &[]
        };

        // Create collapse button or empty space
        let collapse_element: Element<'_, Message> = if is_collapsible {
            button(
                svg(svg::Handle::from_memory(collapse_button_icon))
                    .style(styles::svg_style_secondary),
            )
            .width(15)
            .padding(0)
            .style(button::secondary)
            .on_press(Message::ToggleJsonSection(idx)) // Attach the message
            .into()
        } else {
            text(" ").width(15).into()
        };

        let indented_row = row![
            // Collapse button or space
            collapse_element,
            // Line number column
            text(format!("{:>3} ", idx + 1))
                .size(12)
                .style(move |theme: &Theme| iced::widget::text::Style {
                    color: Some(theme.extended_palette().background.strong.color),
                    ..Default::default()
                })
                .width(30),
            text(" ".repeat(current_indent * indent_size)), // Use current_indent
            if is_collapsible && is_collapsed {
                // Get the pre-calculated line count for this collapsed section
                let count = collapse_counts.get(&idx).copied().unwrap_or(0);
                // Determine opening and closing chars (we only need the closing one now for the indicator)
                let closing_char = if trimmed_line.ends_with('{') { "}" } else { "]" };
                // Get the colors for the tokens and the count text
                let token_color = color_for_token(closing_char, false, false, theme);
                let count_color = theme.extended_palette().background.strong.color;

                // Create the count indicator: { N lines } or [ N lines ]
                // Note: The opening brace/bracket is already part of row_element
                let count_indicator = row![
                    text(format!(" {} lines ", count))
                        .style(move |_| iced::widget::text::Style { color: Some(count_color) }),
                    text(closing_char)
                        .style(move |_| iced::widget::text::Style { color: Some(token_color) })
                ];

                // Show the original row_element followed by the count indicator
                row![row_element, count_indicator]
            } else {
                row_element
            }
        ];

        elements.push(indented_row.into());

        if is_collapsible && is_collapsed {
            skip_depth = Some(current_indent); // Start skipping lines
            // We still need to track indentation *as if* it wasn't collapsed
            if trimmed_line.ends_with('{') || trimmed_line.ends_with('[') {
                indent_level += 1;
            }
        } else if trimmed_line.ends_with('{') || trimmed_line.ends_with('[') {
            indent_level += 1;
        }
    }

    column(elements).spacing(2).into()
}

