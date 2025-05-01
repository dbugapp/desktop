use iced::widget::{container, svg};
use iced_core::Theme;

pub(crate) fn svg_style_primary(theme: &Theme, _status: svg::Status) -> svg::Style {
    svg::Style {
        color: theme.extended_palette().secondary.base.text.into(),
    }
}

pub(crate) fn svg_style_secondary(theme: &Theme, _status: svg::Status) -> svg::Style {
    svg::Style {
        color: theme.extended_palette().secondary.base.text.into(),
    }
}

pub(crate) fn container_code(theme: &Theme) -> container::Style {
    {
        let palette = theme.extended_palette();
        let mut bg_color = palette.secondary.strong.color;
        bg_color.a = 0.05;
        let mut border_color = palette.secondary.strong.color;
        border_color.a = 0.3;

        container::Style {
            background: Some(bg_color.into()),
            border: iced_core::border::rounded(5).color(border_color).width(1.0),
            ..container::Style::default()
        }
    }
}

pub(crate) fn container_modal(theme: &Theme) -> container::Style {
    {
        let palette = theme.extended_palette();
        let bg_color = palette.background.base.color;
        let border_color = palette.background.strong.color;

        container::Style {
            background: Some(bg_color.into()),
            border: iced_core::border::rounded(5).color(border_color).width(1.0),
            ..container::Style::default()
        }
    }
}
pub(crate) fn container_code_closed(theme: &Theme) -> container::Style {
    {
        let palette = theme.extended_palette();
        let mut bg_color = palette.secondary.strong.color;
        bg_color.a = 0.2;
        let mut border_color = palette.secondary.strong.color;
        border_color.a = 0.3;

        container::Style {
            background: Some(bg_color.into()),
            border: iced_core::border::rounded(5).color(border_color).width(1.0),
            ..container::Style::default()
        }
    }
}
