use eframe::*;

mod startup_routine {
    use std::{
        env::{self, VarError},
        fs,
        path::Path,
    };
    fn get_home_path() -> Result<String, VarError> {
        env::var("HOME")
    }
    pub fn run() -> anyhow::Result<()> {
        create_storage_directory()?;
        create_storage_file()?;
        Ok(())
    }
    fn check_if_storage_directory_exists(home_path: &str) -> bool {
        Path::new(&format!("{}/.leitner_box_rs", home_path)).is_dir()
    }
    fn create_storage_directory() -> anyhow::Result<()> {
        let home_path = get_home_path()?;
        if !check_if_storage_directory_exists(&home_path) {
            fs::create_dir(&format!("{}/.leitner_box_rs", home_path))?;
        }
        Ok(())
    }
    fn check_if_storage_file_exists(home_path: &str) -> bool {
        Path::new(&format!("{}/.leitner_box_rs/data.toml", home_path)).is_file()
    }
    fn create_storage_file() -> anyhow::Result<()> {
        let home_path = get_home_path()?;
        if !check_if_storage_file_exists(&home_path) {
            fs::write(&format!("{}/.leitner_box_rs/data.toml", home_path), "")?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init();
    match startup_routine::run() {
        Err(e) => {
            eprintln!("{}", e);
            let error_app = MessageDisplayer {
                error_message: format!("{}", e),
            };
            let options = eframe::NativeOptions {
                ..Default::default()
            };
            eframe::run_native("Leitners box", options, Box::new(|_cc| Box::new(error_app)))
        }
        Ok(_) => {
            let options = eframe::NativeOptions {
                min_window_size: Some(egui::Vec2::new(640., 480.)),
                ..Default::default()
            };
            eframe::run_native(
                "Leitners box",
                options,
                Box::new(|_cc| Box::<LeitnerBox>::default()),
            )
        }
    }
}

struct MessageDisplayer {
    error_message: String,
}
impl eframe::App for MessageDisplayer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| ui.heading(&self.error_message));
    }
}

struct LeitnerBox {}

impl Default for LeitnerBox {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for LeitnerBox {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World");
        });
    }
}
