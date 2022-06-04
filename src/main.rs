#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui::{self, widgets, Ui, Image}, emath::Pos2, epaint::{Stroke, Rounding, ColorImage, ImageData, Color32, TextureHandle, Shape}};

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.vsync = false;
    let App = RustOfLife::default();
    eframe::run_native(
        "Rust of Life",
        options,
        Box::new(|_cc| Box::new(App)),
    );
}

struct RustOfLife {
    width: usize,
    height: usize,
    texture: Option<egui::TextureHandle>,
    frames: usize,
    starttime: std::time::SystemTime,
}

impl Default for RustOfLife {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            texture: None,
            frames: 0,
            starttime: std::time::SystemTime::now()
        }
    }
}

impl eframe::App for RustOfLife {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left side panel").show(ctx, |ui| {
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
            if ui.button("Start Game").clicked() {
                self.init(ctx);
            }
        });



        egui::CentralPanel::default().show(ctx, |ui| {

            match &self.texture {
                Some(tex) => {
                    ui.image(tex, tex.size_vec2());
                },
                None => (),
            }
        });

        self.print_fps();
        ctx.request_repaint();
    }
}

impl RustOfLife{
    
    fn init(&mut self, ctx: &egui::Context)
    {
        let clr_img: ColorImage = ColorImage::new([self.width, self.height], Color32::BLACK);
        let img_data = ImageData::Color(clr_img);
        
        self.texture = Some(ctx.load_texture("game_img", img_data));
    }

    fn print_fps(&mut self)
    {
        self.frames = self.frames + 1;
        
        match self.starttime.elapsed() {
            Ok(runtime) => {
                let time =  std::cmp::max(runtime.as_secs(), 1);

                let fps = (self.frames as u64) / time;
                println!("Current FPS: {}", fps);

            },
            Err(err) => println!("stuff went wrong while calculating fps {}", err),
        }
    }
}
