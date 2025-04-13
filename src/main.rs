use txt_ml::TextEditor;
use eframe::{egui, NativeOptions};
use anyhow::Result;

/// Main entry point for the TXT_ML editor.
/// 
/// Note: When running on Wayland, you may encounter buffer size errors.
/// To avoid these issues, run the application with X11 by setting the
/// environment variable: WINIT_UNIX_BACKEND=x11
#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    // Initialize logging
    env_logger::init();
    
    // Create the editor
    let editor = TextEditor::new();
    
    // Run the application
    let options = NativeOptions {
        // Ensure all dimensions are even numbers
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        min_window_size: Some(egui::vec2(400.0, 300.0)),
        max_window_size: Some(egui::vec2(1920.0, 1080.0)),
        resizable: true,
        transparent: false,
        vsync: true,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        // Wayland-specific options
        decorated: true,
        maximized: false,
        fullscreen: false,
        always_on_top: false,
        // Disable drag and drop to avoid Wayland buffer issues
        drag_and_drop_support: false,
        ..Default::default()
    };
    
    eframe::run_native(
        "TXT_ML Editor",
        options,
        Box::new(|_cc| Box::new(editor)),
    )
}
