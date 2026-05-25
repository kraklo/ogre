use ogre::card::{Card, CardJson};
use std::fs::{self, read_to_string};
use uuid::Uuid;

pub fn parse_decklist(owner_id: Uuid, filepath: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for line in read_to_string(filepath).unwrap().lines() {
        let split_line: Vec<&str> = line.split(" ").collect();
        let num: u32 = split_line[0].parse().unwrap();
        let card = split_line[1..split_line.len()].join(" ");
        let card_name = card_name_to_filename(&card);

        println!("../cards/data/{card_name}.json");
        let card_data =
            fs::read(format!("../cards/data/{card_name}.json")).expect("file reading error.");
        let deserialized_card: CardJson = serde_json::from_slice(&card_data).unwrap();

        for _ in 0..num {
            cards.push(Card::from_card_json(owner_id, deserialized_card.clone()));
        }
    }

    cards
}

fn card_name_to_filename(card_name: &str) -> String {
    card_name.to_lowercase().replace(" ", "_")
}
