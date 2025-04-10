use crate::theme::theme_utils;
use iced::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Light,
    Dark,
}

pub struct Theme {
    pub mode: Mode,

    // Colors
    pub primary: Color,
    pub secondary: Color,
    pub info: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub tertiary: Color,
    pub text: Color,
    pub text_highlighted: Color,
    pub text_toned: Color,
    pub text_muted: Color,
    pub text_dimmed: Color,
    pub border: Color,
    pub border_muted: Color,
    pub border_accented: Color,
    pub border_inverted: Color,
    pub bg: Color,
    pub bg_muted: Color,
    pub bg_elevated: Color,
    pub bg_accented: Color,
}

impl Theme {
    pub fn new(mode: Mode) -> Self {
        match mode {
            Mode::Light => Self::light(),
            Mode::Dark => Self::dark(),
        }
    }

    pub fn light() -> Self {
        Self {
            mode: Mode::Light,
            primary: theme_utils::oklch(0.606, 0.25, 292.717),
            secondary: theme_utils::oklch(0.685, 0.169, 237.323),
            info: theme_utils::oklch(0.623, 0.214, 259.815),
            success: theme_utils::oklch(0.723, 0.219, 149.579),
            warning: theme_utils::oklch(0.795, 0.184, 86.047),
            error: theme_utils::oklch(0.637, 0.237, 25.331),
            tertiary: theme_utils::oklch(0.696, 0.17, 162.48),
            text: theme_utils::oklch(0.37, 0.013, 285.805),
            text_highlighted: theme_utils::oklch(0.37, 0.013, 285.805),
            text_toned: theme_utils::oklch(0.442, 0.017, 285.786),
            text_muted: theme_utils::oklch(0.552, 0.016, 285.938),
            text_dimmed: theme_utils::oklch(0.705, 0.015, 286.067),
            border: theme_utils::oklch(0.92, 0.004, 286.32),
            border_muted: theme_utils::oklch(0.92, 0.004, 286.32),
            border_accented: theme_utils::oklch(0.871, 0.006, 286.286),
            border_inverted: theme_utils::oklch(0.21, 0.006, 285.885),
            bg: theme_utils::hex_to_color("#fff"),
            bg_muted: theme_utils::oklch(0.985, 0.0, 0.0),
            bg_elevated: theme_utils::oklch(0.967, 0.001, 286.375),
            bg_accented: theme_utils::oklch(0.92, 0.004, 286.32),
        }
    }

    pub fn dark() -> Self {
        Self {
            mode: Mode::Dark,
            primary: theme_utils::oklch(0.702, 0.183, 293.541),
            secondary: theme_utils::oklch(0.746, 0.16, 232.661),
            info: theme_utils::oklch(0.707, 0.165, 254.624),
            success: theme_utils::oklch(0.792, 0.209, 151.711),
            warning: theme_utils::oklch(0.852, 0.199, 91.936),
            error: theme_utils::oklch(0.704, 0.191, 22.216),
            tertiary: theme_utils::oklch(0.765, 0.177, 163.223),
            bg: theme_utils::oklch(0.21, 0.006, 285.885),
            text: theme_utils::oklch(0.92, 0.004, 286.32),
            text_highlighted: theme_utils::hex_to_color("#fff"),
            text_toned: theme_utils::oklch(0.871, 0.006, 286.286),
            text_muted: theme_utils::oklch(0.705, 0.015, 286.067),
            text_dimmed: theme_utils::oklch(0.552, 0.016, 285.938),
            border: theme_utils::oklch(0.274, 0.006, 286.033),
            border_muted: theme_utils::oklch(0.37, 0.013, 285.805),
            border_accented: theme_utils::oklch(0.37, 0.013, 285.805),
            border_inverted: theme_utils::hex_to_color("#fff"),
            bg_muted: theme_utils::oklch(0.274, 0.006, 286.033),
            bg_elevated: theme_utils::oklch(0.274, 0.006, 286.033),
            bg_accented: theme_utils::oklch(0.37, 0.013, 285.805),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

/*
Usage::
   let theme = Theme::new(Mode::Light); // or Theme::light()
   let button = Button::new("Click me")
       .style(button::Style {
           background: Some(Background::Color(theme.primary)),
           text_color: theme.text,
           ..Default::default()
       });

THEME FROM dbug.app

light:
  ui-primary: oklch(0.606 0.25 292.717);
  ui-secondary: oklch(0.685 0.169 237.323);
  ui-info: oklch(0.623 0.214 259.815);
  ui-success: oklch(0.723 0.219 149.579);
  ui-warning: oklch(0.795 0.184 86.047);
  ui-error: oklch(0.637 0.237 25.331);
  ui-tertiary: oklch(0.696 0.17 162.48);
  ui-text: oklch(0.37 0.013 285.805);
  ui-text-highlighted: oklch(0.37 0.013 285.805);
  ui-text-toned: oklch(0.442 0.017 285.786);
  ui-text-muted: oklch(0.552 0.016 285.938);
  ui-text-dimmed: oklch(0.705 0.015 286.067);
  ui-border: oklch(0.92 0.004 286.32);
  ui-border-muted: oklch(0.92 0.004 286.32);
  ui-border-accented: oklch(0.871 0.006 286.286);
  ui-border-inverted: oklch(0.21 0.006 285.885);
  ui-bg: #fff;
  ui-bg-muted: oklch(0.985 0 0);
  ui-bg-elevated: oklch(0.967 0.001 286.375);
  ui-bg-accented: oklch(0.92 0.004 286.32);

dark:
  ui-primary: oklch(0.702 0.183 293.541);
  ui-secondary: oklch(0.746 0.16 232.661);
  ui-info: oklch(0.707 0.165 254.624);
  ui-success: oklch(0.792 0.209 151.711);
  ui-warning: oklch(0.852 0.199 91.936);
  ui-error: oklch(0.704 0.191 22.216);
  ui-tertiary: oklch(0.765 0.177 163.223);
  ui-bg: oklch(0.21 0.006 285.885);
  ui-text: oklch(0.92 0.004 286.32);
  ui-text-highlighted: #fff;
  ui-text-toned: oklch(0.871 0.006 286.286);
  ui-text-muted: oklch(0.705 0.015 286.067);
  ui-text-dimmed: oklch(0.552 0.016 285.938);
  ui-border: oklch(0.274 0.006 286.033);
  ui-border-muted: oklch(0.37 0.013 285.805);
  ui-border-accented: oklch(0.37 0.013 285.805);
  ui-border-inverted: #fff;
  ui-bg-muted: oklch(0.274 0.006 286.033);
  ui-bg-elevated: oklch(0.274 0.006 286.033);
  ui-bg-accented: oklch(0.37 0.013 285.805);
 */
