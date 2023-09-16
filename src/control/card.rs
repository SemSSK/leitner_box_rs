use std::time::{SystemTime, UNIX_EPOCH};

use super::box_number::BoxNumber;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AnswerType {
    Correct,
    Wrong,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Default)]
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

#[cfg(test)]
mod test {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::control::box_number::BoxNumber;

    use super::Card;

    fn helper_generate_card() -> Card {
        Card {
            id: 0,
            question: String::new(),
            answer: String::new(),
            current_box: BN::Box1,
            showing_date: 0,
        }
    }

    type BN = BoxNumber;
    #[test]
    fn on_answer_1() {
        let mut card = helper_generate_card();
        card.on_answer(super::AnswerType::Correct);
        assert_eq!(
            Card {
                current_box: BN::Box2,
                showing_date: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    + BN::Box2.get_next_wait_time(),
                ..card.clone()
            },
            card
        );
    }

    #[test]
    fn on_answer_2() {
        let mut card = helper_generate_card();
        card.on_answer(super::AnswerType::Wrong);
        assert_eq!(
            Card {
                current_box: BN::Box1,
                showing_date: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    + BN::Box1.get_next_wait_time(),
                ..card.clone()
            },
            card
        );
    }
}
