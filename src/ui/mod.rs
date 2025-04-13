use egui::Context;
use crate::TextEditor;

#[derive(Clone)]
pub struct UIState {
    pub show_settings: bool,
    pub show_plugins: bool,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            show_settings: false,
            show_plugins: false,
        }
    }

    pub fn render_top_panel(&mut self, ctx: &Context, editor: &mut TextEditor) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // TODO: Implement file open dialog
                    }
                    if ui.button("Save").clicked() {
                        if let Err(e) = editor.save_current_tab() {
                            eprintln!("Error saving file: {}", e);
                        }
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Settings").clicked() {
                        self.show_settings = true;
                    }
                    if ui.button("Plugins").clicked() {
                        self.show_plugins = true;
                    }
                });
            });
        });
    }

    pub fn render_main_panel(&mut self, ctx: &Context, editor: &mut TextEditor) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tab) = editor.tabs.get_mut(editor.current_tab) {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(egui::TextEdit::multiline(&mut tab.content)
                        .desired_width(f32::INFINITY)
                        .desired_rows(30));
                });
            }
        });
    }
} 