use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub fn highlight_json(json: &str) -> String {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("json").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    let mut highlighted = String::new();
    for line in LinesWithEndings::from(json) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        highlighted.push_str(&as_24_bit_terminal_escaped(&ranges[..], true));
    }

    highlighted
}
