use crate::model::*;
use serenity::framework::standard::CommandError;

pub fn character_by_id(client: &reqwest::Client, id: usize) -> Result<Character, CommandError> {
    let resp: LodestoneIdResult = client.get(&format!("https://xivapi.com/character/{}", id))
        .send()?.json()?;
    Ok(resp.character)
}

pub fn search_character(client: &reqwest::Client, name: String, server: Option<String>) -> Result<LodestoneSearchResult, CommandError> {
    let server_query = server.map(|s| format!("&server={}", s)).unwrap_or("".to_string());
    let results = client.get(&format!("https://xivapi.com/character/search?name={}{}", name, server_query))
            .send()?.json()?;
    Ok(results)
}

pub fn character_by_name(client: &reqwest::Client, name: String, server: Option<String>) -> Result<Character, CommandError> {
    let search = search_character(client, name, server)?;
    if search.pagination.results_total > 1 {
        Err(CommandError(format!("Search not specific enough, found {} matching characters", search.pagination.results_total)))
    } else if search.results.is_empty() {
        Err(CommandError("No matching character found, try again!".to_string()))
    } else {
        let raw_char = search.results.first().expect("character");
        character_by_id(client, raw_char.id)
    }
}