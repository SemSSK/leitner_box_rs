use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::startup_routine;

use super::card::Card;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn get_deck() -> anyhow::Result<Self> {
        let deck_json = fs::read_to_string(format!(
            "{}/.leitner_box_rs/data.json",
            startup_routine::get_home_path()?
        ))?;
        let cards: Vec<Card> = serde_json::from_str(&deck_json)?;
        Ok(Self(cards))
    }
    pub fn get_cards(&mut self) -> &mut Vec<Card> {
        &mut self.0
    }
    pub fn get_cards_to_show(&mut self) -> Vec<&mut Card> {
        self.0
            .iter_mut()
            .filter(|card| {
                card.showing_date
                    <= SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
            })
            .collect()
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
