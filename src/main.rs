mod deck;
mod startup_routine;
mod utils;

use std::fmt::format;

use deck::{
    card::{AnswerType, Card},
    deck::Deck,
};
use eframe::{egui::Layout, *};

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
                current_state: State::Neutral,
                card_template: Card::default(),
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
    AddCard,
    UpdateCard,
    AnswerCard(AnswerType),
}

struct LeitnerBox {
    current_state: State,
    deck: Deck,
    card_template: Card,
}

impl eframe::App for LeitnerBox {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("Right Menu").show(ctx, |ui| {
            ui.heading("📭 Leitner's Box Menu");
            ui.separator();
            ui.vertical_centered_justified(|ui| {
                if ui.button("📝 Show Deck").clicked() {
                    self.current_state = State::ShowDeck;
                };
                if ui.button("？ Start Quiz").clicked() {
                    self.current_state = State::ShowCardsToAnswer;
                };
                if ui.button("⊞ Add Card").clicked() {
                    self.card_template = Card::default();
                    self.current_state = State::AddCard;
                };
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| match self.current_state {
            State::Neutral => {
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Ready to study ?");
                });
            }
            State::ShowDeck => {
                let mut marked_for_delete = None;
                egui::Grid::new("show deck grid")
                    .num_columns(5)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("ID");
                        ui.label("Question");
                        ui.label("Answer");
                        ui.label("Current Box");
                        ui.label("Delete");
                        ui.end_row();
                        self.deck.get_cards().iter().for_each(|card| {
                            ui.label(card.id.to_string());
                            ui.label(&card.question);
                            ui.label(&card.answer);
                            ui.label(card.current_box.show());
                            if ui.button("delete").clicked() {
                                marked_for_delete = Some(card.id);
                            }
                            ui.end_row();
                        });
                    });
                if let Some(id) = marked_for_delete {
                    self.deck.delete_card(id);
                    self.deck.save().expect("saving deck error");
                };
            }
            State::AddCard => {
                egui::Grid::new("Add card grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Question");
                        ui.text_edit_multiline(&mut self.card_template.question);
                        ui.end_row();
                        ui.label("Answer");
                        ui.text_edit_multiline(&mut self.card_template.answer);
                        ui.end_row();
                        ui.label("Starting box");
                        ui.label(self.card_template.current_box.show());
                        ui.end_row();
                        if ui.button("Clear").clicked() {
                            self.card_template = Card::default();
                        }
                        if ui.button("Save").clicked() {
                            self.deck.add_card(self.card_template.clone());
                            self.current_state = State::ShowDeck;
                            self.deck.save().expect("saving deck error");
                        }
                    });
            }
            _ => (),
        });
    }
}
