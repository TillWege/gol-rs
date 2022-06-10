use eframe::egui::{self, widgets, };

pub struct Settings {
    pub height: isize,
    pub width: isize,
    pub zoom: isize
}

impl Default for Settings {
    fn default() -> Self {
        Self { 
            height: 100, 
            width: 100,
            zoom: 1 
        }
    }
}

impl Settings {
    pub fn show<F: FnOnce(&mut Self)>(&mut self, ctx: &egui::Context, on_start_click: F){
        egui::SidePanel::left("settings panel").show(ctx, |ui| {
            ui.heading("Rust of Life");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Height:");
                ui.add(widgets::DragValue::new(&mut self.height));
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.add(widgets::DragValue::new(&mut self.width));
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Zoom:");
                ui.add(widgets::DragValue::new(&mut self.zoom));
            });
            ui.separator();
            if ui.button("Start Game").clicked() {
                on_start_click(self);
            }
        });
    }
}