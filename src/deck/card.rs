use std::time::{SystemTime, UNIX_EPOCH};

use super::box_number::BoxNumber;

pub enum AnswerType {
    Correct,
    Wrong,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Card {
    pub id: usize,
    pub question: String,
    pub answer: String,
    pub current_box: BoxNumber,
    pub showing_date: u128,
}

impl Card {
    pub fn on_answer(&mut self, answer: AnswerType) {
        match answer {
            AnswerType::Correct => self.current_box = self.current_box.move_to_next(),
            AnswerType::Wrong => self.current_box = BoxNumber::Box1,
        }
        self.showing_date = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            + self.current_box.get_next_wait_time()
    }
}
