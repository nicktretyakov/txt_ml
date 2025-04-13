pub mod ai;
pub mod config;
pub mod plugins;
pub mod syntax;
pub mod ui;

use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::RwLock;
use crate::config::EditorConfig;
use crate::ui::UIState;
use crate::ai::AIManager;
use crate::plugins::PluginManager;
use eframe::egui;

pub struct TextEditor {
    pub tabs: Vec<Tab>,
    pub current_tab: usize,
    pub config: Arc<RwLock<EditorConfig>>,
    pub ui_state: UIState,
    pub ai_manager: AIManager,
    pub plugin_manager: Arc<RwLock<PluginManager>>,
}

#[derive(Default)]
pub struct Tab {
    pub path: Option<PathBuf>,
    pub content: String,
}

impl Tab {
    pub fn new() -> Self {
        Self {
            path: None,
            content: String::new(),
        }
    }

    pub fn title(&self) -> String {
        self.path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string()
    }
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            current_tab: 0,
            config: Arc::new(RwLock::new(EditorConfig::default())),
            ui_state: UIState::new(),
            ai_manager: AIManager::new(),
            plugin_manager: Arc::new(RwLock::new(PluginManager::new())),
        }
    }

    pub fn open_file(&mut self, path: PathBuf) -> std::io::Result<()> {
        let content = std::fs::read_to_string(&path)?;
        self.tabs.push(Tab {
            path: Some(path),
            content,
        });
        self.current_tab = self.tabs.len() - 1;
        Ok(())
    }

    pub fn save_current_tab(&self) -> std::io::Result<()> {
        if let Some(tab) = self.tabs.get(self.current_tab) {
            if let Some(path) = &tab.path {
                std::fs::write(path, &tab.content)?;
            }
        }
        Ok(())
    }

    pub fn set_theme(&mut self, theme: crate::config::EditorTheme) {
        self.config.write().theme = theme;
    }

    pub fn get_theme_css(&self) -> String {
        self.config.read().get_theme_css()
    }
}

impl eframe::App for TextEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut ui_state = self.ui_state.clone();
        ui_state.render_top_panel(ctx, self);
        ui_state.render_main_panel(ctx, self);
    }
}

// Public API functions
impl TextEditor {
    // Get AI completion for the current tab
    pub async fn get_ai_completion(&self) -> Result<String, anyhow::Error> {
        if let Some(tab) = self.tabs.get(self.current_tab) {
            self.ai_manager.request_completion(&tab.content).await
        } else {
            Ok(String::new())
        }
    }
    
    // Process text through plugins
    pub fn process_through_plugins(&self, text: &str) -> String {
        self.plugin_manager.read().process_text(text)
    }
    
    // Get available plugin commands
    pub fn get_plugin_commands(&self) -> Vec<String> {
        self.plugin_manager.read().get_commands()
    }
} 