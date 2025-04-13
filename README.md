# TXT_ML - AI-Powered Text Editor

A modern text editor with AI capabilities, built in Rust. Features include syntax highlighting, Markdown preview, AI code completion, and a plugin system.

## Features

- **Cross-platform GUI** using egui + eframe
- **Syntax highlighting** with syntect
- **AI-powered code completion** using DeepSeek API
- **Plugin system** for extending functionality
- **Markdown preview** with MathJax support
- **Multiple themes** (Dark, Light, Sepia)
- **File management** with tabs
- **Search and replace** functionality
- **Auto-save** capability
- **Customizable key bindings**

## Dependencies

- egui + eframe - Cross-platform GUI
- syntect - Syntax highlighting
- tokio - Async runtime
- reqwest - HTTP client for AI API
- serde + toml - Configuration
- rfd - File dialogs
- libloading - Plugin system
- pulldown-cmark - Markdown parsing
- egui_html - HTML rendering
- MathJax - LaTeX rendering

## Installation

1. Clone the repository:
  

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the editor:
   ```
   cargo run --release
   ```

## Configuration

The editor can be configured through a TOML file located at `~/.config/txt_ml/config.toml`. Example configuration:

```toml
theme = "dark"
font_size = 14.0
tab_size = 4
auto_save = true
auto_save_interval = 300
plugin_path = "plugins"
```

## Plugins

Plugins can be written in Rust and loaded dynamically. Example plugin:

```rust
use txt_ml::plugins::EditorPlugin;

pub struct MyPlugin;

impl EditorPlugin for MyPlugin {
    fn name(&self) -> &str {
        "my_plugin"
    }
    
    fn process_text(&self, text: &str) -> String {
        // Process text
        text.to_string()
    }
    
    fn get_commands(&self) -> Vec<String> {
        vec!["my_command".to_string()]
    }
}

#[no_mangle]
pub extern "C" fn init_plugin() -> Box<dyn EditorPlugin> {
    Box::new(MyPlugin)
}
```

## AI Integration

The editor integrates with the DeepSeek API for AI-powered code completion. The AI model is automatically selected based on the input characteristics:

- MLP for short inputs
- RNN for multi-line inputs
- Transformer for long-form text

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
