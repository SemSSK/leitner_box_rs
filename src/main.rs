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
                current_state: State::Neutral,
                card_template: Card::default(),
                current_answer: Default::default(),
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
}

struct LeitnerBox {
    current_state: State,
    deck: Deck,
    card_template: Card,
    current_answer: String,
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
                    self.current_answer = String::new();
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
                        ui.label("Update");
                        ui.end_row();
                        self.deck.get_cards().iter().for_each(|card| {
                            ui.label(card.id.to_string());
                            ui.label(&card.question);
                            ui.label(&card.answer);
                            ui.label(card.current_box.show());
                            if ui.button("delete").clicked() {
                                marked_for_delete = Some(card.id);
                            }
                            if ui.button("update").clicked() {
                                self.card_template = card.clone();
                                self.current_state = State::UpdateCard;
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
            State::UpdateCard => {
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
                            self.deck.update_card(self.card_template.clone());
                            self.current_state = State::ShowDeck;
                            self.deck.save().expect("saving deck error");
                        }
                    });
            }
            State::ShowCardsToAnswer => {
                let cards = self.deck.get_cards_to_show();
                let card_option = cards.into_iter().next();
                let mut save = false;
                match card_option {
                    Some(card) => {
                        egui::Grid::new("Answer grid")
                            .num_columns(2)
                            .show(ui, |ui| {
                                ui.label("Question");
                                ui.label(&card.question);
                                ui.end_row();
                                ui.label("Your Answer");
                                ui.text_edit_multiline(&mut self.current_answer);
                                ui.end_row();
                                if ui.label("Real Answer").hovered() {
                                    ui.label(&card.answer);
                                };
                                ui.end_row();
                                if ui.button("Correct").clicked() {
                                    card.on_answer(AnswerType::Correct);
                                    save = true;
                                };
                                if ui.button("wrong").clicked() {
                                    card.on_answer(AnswerType::Wrong);
                                    save = true;
                                };
                            });
                    }
                    None => {
                        self.current_state = State::Neutral;
                    }
                };
                if save {
                    self.deck.save().expect("Deck save error");
                }
            }
            _ => (),
        });
    }
}
