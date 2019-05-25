use serde::{Deserialize, Serialize};
use serenity::model::id::UserId;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterId(pub usize);

pub type CharacterListings = HashMap<UserId, CharacterId>;

#[derive(Serialize, Deserialize, Debug)]
pub struct BotfloxStorage {
    pub listings: CharacterListings,
}

impl BotfloxStorage {
    pub fn new() -> BotfloxStorage {
        BotfloxStorage {
            listings: CharacterListings::new(),
        }
    }
}
