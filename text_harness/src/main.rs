use ogre;
use uuid::Uuid;

use crate::decklist_parser::parse_decklist;

mod decklist_parser;

fn main() {
    let decklist = parse_decklist(Uuid::new_v4(), "decklists/test_decklist.txt");
    let game_state = ogre::new_game(vec![
        (String::from("Bob"), decklist.clone()),
        (String::from("Alice"), decklist.clone()),
    ]);
    println!("Seed: {}", hex::encode(game_state.rng.get_seed()));
    println!("{:?}", game_state)
}
