#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod game;
mod settings;

use eframe::{
    egui::{self, },
    epaint::{Color32, ColorImage, ImageData},
};
use game::Game;
use settings::Settings;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.vsync = true;
    let app = RustOfLife::default();
     eframe::run_native("Rust of Life", 
                         options, 
                         Box::new(|_cc| Box::new(app)));
}

struct RustOfLife {
    game: Option<Game>,
    texture: Option<egui::TextureHandle>,
    frames: usize,
    starttime: std::time::SystemTime,
    settings: Settings,
}

impl Default for RustOfLife {
    fn default() -> Self {
        Self {
            game: None,
            texture: None,
            frames: 0,
            starttime: std::time::SystemTime::now(),
            settings: Settings::default(),
        }
    }
}

impl eframe::App for RustOfLife {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.settings.show(ctx, |settings|{
            self.game = Some(Game::new(settings.width, settings.height));
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

    fn update_texture(&mut self, ctx: &egui::Context){
        if self.game.is_none() { return };

        let game_ref = self.game.as_mut().unwrap();
        
        let img_width: usize   = (game_ref.width * self.settings.zoom).try_into().unwrap();
        let img_height: usize  = (game_ref.height * self.settings.zoom).try_into().unwrap();
        let zoom_offset: usize = (self.settings.zoom.pow(2)).try_into().unwrap();

        let mut clr_img: ColorImage = ColorImage::new([img_width, img_height], Color32::BLACK);
        
        
        for i in 0..clr_img.pixels.len() {
            let val = game_ref.get_value_by_index(i / zoom_offset);
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
