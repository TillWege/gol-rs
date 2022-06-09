#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod game;

use eframe::{
    egui::{self, widgets, },
    epaint::{Color32, ColorImage, ImageData},
};
use game::Game;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.vsync = true;
    let app = RustOfLife::default();
     eframe::run_native("Rust of Life", 
                         options, 
                         Box::new(|_cc| Box::new(app)));
}

struct RustOfLife {
    width: usize,
    height: usize,
    game: Option<Game>,
    texture: Option<egui::TextureHandle>,
    frames: usize,
    starttime: std::time::SystemTime,
}

impl Default for RustOfLife {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            game: None,
            texture: None,
            frames: 0,
            starttime: std::time::SystemTime::now(),
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
                self.init_game();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| match &self.texture {
            Some(tex) => {
                ui.image(tex, tex.size_vec2());
            }
            None => (),
        });

        if self.game.is_some() {
            self.game.as_mut().unwrap().update();
            self.update_texture(ctx);
        }

        self.print_fps();
        ctx.request_repaint();
    }
}

impl RustOfLife {
    fn init_game(&mut self) {
        let i_width  = isize::try_from(self.width).ok().unwrap_or_else(||0);
        let i_height = isize::try_from(self.height).ok().unwrap_or_else(||0);
        
        self.game = Some(Game::new(i_width, i_height));
    }


    fn update_texture(&mut self, ctx: &egui::Context){
        if self.game.is_none() { return };
        
        let mut clr_img: ColorImage = ColorImage::new([self.width, self.height], Color32::BLACK);
        //let pixels = &clr_img.pixels;
        //self.game.as_mut().unwrap().show(&mut pixels);
        for i in 0..clr_img.pixels.len() {
            let val = self.game.as_mut().unwrap().get_value_by_index(i);
            if val.unwrap_or_else(||false) {
                clr_img.pixels[i] = Color32::BLACK;
            }else{
                clr_img.pixels[i] = Color32::WHITE;
            }
        }

        let img_data = ImageData::Color(clr_img);

        self.texture = Some(ctx.load_texture("game_img", img_data));
    }


    fn print_fps(&mut self) {
        self.frames = self.frames + 1;

        match self.starttime.elapsed() {
            Ok(runtime) => {
                let time = std::cmp::max(runtime.as_secs(), 1);

                let fps = (self.frames as u64) / time;
                println!("Current FPS: {}", fps);
            }
            Err(err) => println!("stuff went wrong while calculating fps {}", err),
        }
    }
}
