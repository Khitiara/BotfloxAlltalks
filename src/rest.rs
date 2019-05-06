use crate::model::*;

use std::fmt;

#[derive(Debug)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {

}

pub fn character_by_id(client: &reqwest::Client, id: usize) -> Result<Character, Box<std::error::Error>> {
    let resp: LodestoneIdResult = client.get(&format!("https://xivapi.com/character/{}", id))
        .send()?.json()?;
    Ok(resp.character)
}

pub fn search_character(client: &reqwest::Client, name: String, server: Option<String>) -> Result<Vec<RawCharacter>, Box<std::error::Error>> {
    let server_query = server.map(|s| format!("&server={}", s)).unwrap_or("".to_string());
    let resp: LodestoneSearchResult =
        client.get(&format!("https://xivapi.com/character/search?name={}{}", name, server_query))
            .send()?.json()?;
    Ok(resp.results)
}

pub fn character_by_name(client: &reqwest::Client, name: String, server: Option<String>) -> Result<Character, Box<std::error::Error>> {
    let search = search_character(client, name, server)?;
    if search.len() > 1 {
        Err(Box::new(Error(format!("Search not specific enough, found {} matching characters", search.len()))))
    } else if search.is_empty() {
        Err(Box::new(Error("No matching character found, try again!".to_string())))
    } else {
        let raw_char = search.first().expect("character");
        character_by_id(client, raw_char.id)
    }
}