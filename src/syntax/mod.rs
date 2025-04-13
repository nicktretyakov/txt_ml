use syntect::{
    parsing::SyntaxSet,
    highlighting::{ThemeSet, Theme},
    easy::HighlightLines,
};
use std::collections::HashMap;

// Syntax highlighter
pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    theme: Theme,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set.themes["base16-ocean.dark"].clone();
        
        Self {
            syntax_set,
            theme_set,
            theme,
        }
    }

    pub fn highlight(&self, text: &str, syntax_name: &str) -> String {
        let syntax = self.syntax_set.find_syntax_by_name(syntax_name)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
            
        let mut h = HighlightLines::new(syntax, &self.theme);
        let mut result = String::new();
        
        for line in text.lines() {
            let ranges: Vec<(syntect::highlighting::Style, &str)> = h.highlight_line(line, &self.syntax_set).unwrap_or_default();
            
            for (style, text) in ranges {
                let color = style.foreground;
                result.push_str(&format!(
                    "<span style=\"color: rgb({}, {}, {})\">{}</span>",
                    color.r,
                    color.g,
                    color.b,
                    text.replace('<', "&lt;").replace('>', "&gt;")
                ));
            }
            result.push('\n');
        }
        
        result
    }
    
    // Get available syntaxes
    pub fn get_available_syntaxes(&self) -> Vec<String> {
        self.syntax_set.syntaxes()
            .iter()
            .map(|s| s.name.clone())
            .collect()
    }
    
    // Get available themes
    pub fn get_available_themes(&self) -> Vec<String> {
        self.theme_set.themes.keys()
            .map(|s| s.to_string())
            .collect()
    }
}

// Syntax detection
pub struct SyntaxDetector {
    syntax_set: SyntaxSet,
    extension_map: HashMap<String, String>,
}

impl SyntaxDetector {
    pub fn new() -> Self {
        let mut extension_map = HashMap::new();
        extension_map.insert("rs".to_string(), "Rust".to_string());
        extension_map.insert("md".to_string(), "Markdown".to_string());
        extension_map.insert("txt".to_string(), "Plain Text".to_string());
        
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            extension_map,
        }
    }
    
    // Detect syntax from file extension
    pub fn detect_from_extension(&self, extension: &str) -> String {
        self.extension_map.get(extension)
            .cloned()
            .unwrap_or_else(|| "Plain Text".to_string())
    }
    
    // Detect syntax from file content
    pub fn detect_from_content(&self, content: &str) -> String {
        // Use syntect to detect syntax
        if let Some(syntax) = self.syntax_set.find_syntax_by_extension("rs") {
            if content.contains("fn main()") || content.contains("use ") {
                return syntax.name.clone();
            }
        }
        
        if let Some(syntax) = self.syntax_set.find_syntax_by_extension("md") {
            if content.contains("# ") || content.contains("* ") || content.contains("```") {
                return syntax.name.clone();
            }
        }
        
        "Plain Text".to_string()
    }
} 