mod startup_routine;

use eframe::*;

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

mod cards {

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    enum BoxNumber {
        Box1,
        Box2,
        Box3,
        Box4,
        Box5,
        Box6,
        Box7(u8),
    }

    impl BoxNumber {
        fn move_to_next(&self) -> BoxNumber {
            type BN = BoxNumber;
            match self {
                BN::Box1 => BN::Box2,
                BN::Box2 => BN::Box3,
                BN::Box3 => BN::Box4,
                BN::Box4 => BN::Box5,
                BN::Box5 => BN::Box6,
                BN::Box6 => BN::Box7(0),
                BN::Box7(n) => BN::Box7(n + 1),
            }
        }

        fn get_next_date_to_show(&self) -> u32 {
            todo!()
        }
    }

    enum AnswerType {
        Correct,
        Wrong,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Card {
        id: usize,
        question: String,
        answer: String,
        current_box: BoxNumber,
        showing_date: u32,
    }

    impl Card {
        fn on_answer(&mut self, answer: AnswerType) {
            match answer {
                AnswerType::Correct => {
                    self.current_box = self.current_box.move_to_next();
                }
                AnswerType::Wrong => self.current_box = BoxNumber::Box1,
            }
        }
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Deck(Vec<Card>);

    impl Deck {
        pub fn get_deck() -> Self {
            todo!()
        }
        pub fn get_cards(&mut self) -> &mut Vec<Card> {
            &mut self.0
        }
        pub fn add_card(&mut self, card: Card) {
            let card = Card {
                id: self.0.len(),
                ..card
            };
            self.0.push(card);
        }
        pub fn delete_card(&mut self, id: usize) {
            self.0.retain(|card| card.id != id);
        }
        pub fn update_card(&mut self, card: Card) {
            for dcard in self.0.iter_mut() {
                if dcard.id == card.id {
                    let _ = std::mem::replace(dcard, card);
                    break;
                }
            }
        }
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
