use std::collections::HashMap;
use serenity::model::id::UserId;
use serde::{Serialize, Deserialize};

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
            listings: CharacterListings::new()
        }
    }
}