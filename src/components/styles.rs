use iced::widget::svg;
use iced_core::Theme;

pub(crate) fn svg_style_primary(theme: &Theme, _status: svg::Status) -> svg::Style {
    svg::Style {
        color: theme.extended_palette().secondary.base.text.into(),
        ..svg::Style::default()
    }
}

pub(crate) fn svg_style_secondary(theme: &Theme, _status: svg::Status) -> svg::Style {
    svg::Style {
        color: theme.extended_palette().secondary.base.text.into(),
        ..svg::Style::default()
    }
}

pub(crate) fn svg_style_danger(theme: &Theme, _status: svg::Status) -> svg::Style {
    svg::Style {
        color: theme.extended_palette().danger.base.color.into(),
        ..svg::Style::default()
    }
}
