use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn highlight(&self, code: &str, language: Option<&str>) -> String {
        // Determine syntax based on language hint or default to Python
        let syntax = language
            .and_then(|lang| self.syntax_set.find_syntax_by_token(lang))
            .or_else(|| self.syntax_set.find_syntax_by_extension("py"))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        // Choose a theme (you can make this configurable)
        let theme = &self.theme_set.themes["base16-ocean.dark"];
        
        let mut highlighter = HighlightLines::new(syntax, theme);
        let mut html = String::new();

        for line in LinesWithEndings::from(code) {
            let ranges: Vec<(Style, &str)> = highlighter
                .highlight_line(line, &self.syntax_set)
                .unwrap_or_default();
            
            let line_html = styled_line_to_highlighted_html(
                &ranges[..],
                IncludeBackground::No,
            )
            .unwrap_or_else(|_| line.to_string());
            
            html.push_str(&line_html);
        }

        html
    }

    pub fn get_background_color(&self) -> String {
        let theme = &self.theme_set.themes["base16-ocean.dark"];
        format!(
            "#{:02x}{:02x}{:02x}",
            theme.settings.background.unwrap_or_default().r,
            theme.settings.background.unwrap_or_default().g,
            theme.settings.background.unwrap_or_default().b
        )
    }
}
