use eframe::*;

fn main() -> Result<()> {
    env_logger::init();
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

struct LeitnerBox {}

impl Default for LeitnerBox {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for LeitnerBox {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World");
        });
    }
}
