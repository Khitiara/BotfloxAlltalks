use serenity::framework::standard::CommandError;

use crate::model::*;

pub fn character_by_id(client: &reqwest::Client, id: usize) -> Result<Character, CommandError> {
    let url = format!("https://xivapi.com/character/{}?extended=true", id);
    println!("{}", url);
    let resp: LodestoneCharacterIdResult = client.get(&url)
        .send()?.json()?;
    let mut character = resp.character;
    if character.fc_id.is_some() {
        let fc = fc_by_id(client, character.fc_id.clone().unwrap())?;
        character.fc = fc;
    }
    Ok(character)
}

pub fn fc_by_id(client: &reqwest::Client, id: String) -> Result<FreeCompany, CommandError> {
    let url = format!("https://xivapi.com/freecompany/{}", id);
    println!("{}", url);
    let resp: LodestoneFCIdResult = client.get(&url)
        .send()?.json()?;
    Ok(resp.free_company)
}

pub fn search_character(client: &reqwest::Client, name: String, server: Option<String>) -> Result<LodestoneSearchResult<RawCharacter>, CommandError> {
    let server_query = server.map(|s| format!("&server={}", s)).unwrap_or("".to_string());
    let results = client.get(&format!("https://xivapi.com/character/search?name={}{}", name, server_query))
        .send()?.json()?;
    Ok(results)
}

pub fn character_by_name(client: &reqwest::Client, name: String, server: Option<String>) -> Result<Character, CommandError> {
    let raw_char = id_by_name(client, name, server)?;
    character_by_id(client, raw_char.id)
}

pub fn id_by_name(client: &reqwest::Client, name: String, server: Option<String>) -> Result<RawCharacter, CommandError> {
    let search = search_character(client, name, server)?;
    if search.pagination.results_total > 1 {
        Err(CommandError(format!("Search not specific enough, found {} matching characters", search.pagination.results_total)))
    } else if search.results.is_empty() {
        Err(CommandError("No matching character found, try again!".to_string()))
    } else {
        let raw_char = search.results.first().expect("character");
        Ok(raw_char.clone())
    }
}

pub fn duty_by_id(client: &reqwest::Client, id: usize) -> Result<DutyInfo, CommandError> {
    let url = format!("https://xivapi.com/InstanceContent/{}?columns=ContentFinderCondition\
    .ClassJobLevelRequired,ContentFinderCondition.ClassJobLevelSync,ContentFinderCondition\
    .ContentMemberType,ContentFinderCondition.ContentType.ID,ContentFinderCondition.ContentType.Name\
    ,ContentFinderCondition.Name,ContentFinderCondition.ItemLevelRequired,ContentFinderCondition\
    .ItemLevelSync,ContentFinderCondition.ID", id);
    let resp: DutyResult = client.get(&url).send()?.json()?;
    Ok(resp.content_finder_condition)
}