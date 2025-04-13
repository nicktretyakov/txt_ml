use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use anyhow::Result;

// Editor configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditorConfig {
    pub theme: EditorTheme,
    pub font_size: f32,
    pub line_numbers: bool,
    pub word_wrap: bool,
    pub tab_size: usize,
    pub auto_indent: bool,
    pub config_path: PathBuf,
    pub auto_save: bool,
    pub auto_save_interval: u64,
    pub plugin_path: PathBuf,
    pub syntax_themes: std::collections::HashMap<String, String>,
    pub key_bindings: KeyBindings,
}

// Editor themes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EditorTheme {
    Dark,
    Light,
    Sepia,
    Custom(String),
}

// Key bindings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyBindings {
    pub save: String,
    pub open: String,
    pub new: String,
    pub close: String,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            theme: EditorTheme::Dark,
            font_size: 14.0,
            line_numbers: true,
            word_wrap: true,
            tab_size: 4,
            auto_indent: true,
            config_path: PathBuf::from("config.toml"),
            auto_save: true,
            auto_save_interval: 300, // 5 minutes
            plugin_path: PathBuf::from("plugins"),
            syntax_themes: Self::load_default_syntax_themes(),
            key_bindings: KeyBindings::default(),
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            save: "Ctrl+S".to_string(),
            open: "Ctrl+O".to_string(),
            new: "Ctrl+N".to_string(),
            close: "Ctrl+W".to_string(),
        }
    }
}

impl EditorConfig {
    // Load configuration from file
    pub fn load() -> Result<Self> {
        let config_path = PathBuf::from("config.toml");
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let mut config: EditorConfig = toml::from_str(&content)?;
            config.config_path = config_path;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }
    
    // Save configuration to file
    pub fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&self.config_path, content)?;
        Ok(())
    }
    
    // Load default syntax themes
    fn load_default_syntax_themes() -> std::collections::HashMap<String, String> {
        let mut themes = std::collections::HashMap::new();
        themes.insert("rust".to_string(), "base16-ocean.dark".to_string());
        themes.insert("markdown".to_string(), "base16-ocean.dark".to_string());
        themes.insert("plaintext".to_string(), "base16-ocean.dark".to_string());
        themes
    }
    
    // Get CSS for the current theme
    pub fn get_theme_css(&self) -> String {
        match &self.theme {
            EditorTheme::Dark => include_str!("../themes/dark.css").to_string(),
            EditorTheme::Light => include_str!("../themes/light.css").to_string(),
            EditorTheme::Sepia => include_str!("../themes/sepia.css").to_string(),
            EditorTheme::Custom(path) => {
                std::fs::read_to_string(path).unwrap_or_else(|_| {
                    eprintln!("Failed to load custom theme: {}", path);
                    include_str!("../themes/dark.css").to_string()
                })
            }
        }
    }
} 