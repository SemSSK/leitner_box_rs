mod deck;
mod startup_routine;
mod utils;

use deck::{
    card::{AnswerType, Card},
    deck::Deck,
};
use eframe::*;

fn main() -> Result<()> {
    env_logger::init();
    match startup_routine::run() {
        Err(e) => {
            eprintln!("{}", e);
            let error_app = ErrorMessageDisplayer {
                error_message: format!("{}", e),
            };
            let options = eframe::NativeOptions {
                ..Default::default()
            };
            eframe::run_native("Leitners box", options, Box::new(|_cc| Box::new(error_app)))
        }
        Ok(_) => {
            let deck = Deck::get_deck().expect("Getting deck error");
            let app = LeitnerBox {
                currentState: State::Neutral,
                deck,
            };
            let options = eframe::NativeOptions {
                min_window_size: Some(egui::Vec2::new(640., 480.)),
                ..Default::default()
            };
            eframe::run_native("Leitners box", options, Box::new(|_cc| Box::new(app)))
        }
    }
}

struct ErrorMessageDisplayer {
    error_message: String,
}
impl eframe::App for ErrorMessageDisplayer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| ui.heading(&self.error_message));
    }
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Neutral,
    ShowDeck,
    ShowCardsToAnswer,
    DeleteCard(usize),
    AddCard(Card),
    UpdateCard(Card),
    SaveDeck(Deck),
    AnswerCard(AnswerType, Card),
}

struct LeitnerBox {
    currentState: State,
    deck: Deck,
}

impl eframe::App for LeitnerBox {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("Right Menu").show(ctx, |ui| {
            ui.heading("📭 Leitner's Box Menu");
            ui.separator();
            ui.vertical_centered_justified(|ui| {
                if ui.button("📝 Show Deck").clicked() {
                    self.currentState = State::ShowDeck;
                };
                if ui.button("？ Start Quiz").clicked() {
                    self.currentState = State::ShowCardsToAnswer;
                };
            })
        });
        let mut t = true;
        egui::Window::new("title")
            .collapsible(true)
            .resizable(true)
            .open(&mut t)
            .show(ctx, |ui| {
                ui.label("test");
            });
        egui::CentralPanel::default().show(ctx, |ui| match self.currentState {
            State::Neutral => ui.label("Neutral"),
            State::ShowDeck => ui.label("Show Deck"),
            State::ShowCardsToAnswer => ui.label("Show Cards to answer"),
            State::DeleteCard(_) => ui.label("Delete Card"),
            State::AddCard(_) => ui.label("Add Card"),
            State::UpdateCard(_) => ui.label("Update Card"),
            State::SaveDeck(_) => ui.label("Save Deck"),
            State::AnswerCard(_, _) => ui.label("Answer Card"),
        });
    }
}
