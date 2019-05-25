use serenity::framework::standard::CommandError;

use crate::model::*;

pub fn character_by_id(client: &reqwest::Client, id: usize) -> Result<Character, CommandError> {
    let url = format!(
        "https://xivapi.com/character/{}?snake_case=1&extended=1",
        id
    );
    println!("{}", url);
    let resp: LodestoneCharacterIdResult = client.get(&url).send()?.json()?;
    let mut character = resp.character;
    if let Some(ref fc_id) = &character.free_company_id {
        character.fc = fc_by_id(client, fc_id)?;
    }
    Ok(character)
}

pub fn fc_by_id(client: &reqwest::Client, id: &str) -> Result<FreeCompany, CommandError> {
    let url = format!("https://xivapi.com/freecompany/{}?snake_case=1", id);
    println!("{}", url);
    let resp: LodestoneFCIdResult = client.get(&url).send()?.json()?;
    Ok(resp.free_company)
}

pub fn search_character(
    client: &reqwest::Client,
    name: &str,
    server: Option<&str>,
) -> Result<LodestoneSearchResult<RawCharacter>, CommandError> {
    let server_query = server
        .map(|s| format!("&server={}", s))
        .unwrap_or("".to_string());
    let results = client
        .get(&format!(
            "https://xivapi.com/character/search?snake_case=1&name={}{}",
            name, server_query
        ))
        .send()?
        .json()?;
    Ok(results)
}

pub fn character_by_name(
    client: &reqwest::Client,
    name: &str,
    server: Option<&str>,
) -> Result<Character, CommandError> {
    let raw_char = id_by_name(client, name, server)?;
    character_by_id(client, raw_char.id)
}

pub fn id_by_name(
    client: &reqwest::Client,
    name: &str,
    server: Option<&str>,
) -> Result<RawCharacter, CommandError> {
    let search = search_character(client, name, server)?;
    if search.pagination.results_total > 1 {
        Err(CommandError(format!(
            "Search not specific enough, found {} matching characters",
            search.pagination.results_total
        )))
    } else if search.results.is_empty() {
        Err(CommandError(
            "No matching character found, try again!".to_string(),
        ))
    } else {
        let raw_char = search.results.first().expect("character");
        Ok(raw_char.clone())
    }
}

pub fn content_search(
    client: &reqwest::Client,
    name: &str,
    content_type: &str,
) -> Result<LodestoneSearchResult<WithIdName>, CommandError> {
    let url = format!(
        "https://xivapi.com/search?snake_case=1&string={}&indexes={}",
        name, content_type
    );
    Ok(client.get(&url).send()?.json()?)
}

pub fn duty_by_id(client: &reqwest::Client, id: usize) -> Result<DutyInfo, CommandError> {
    let url = format!("https://xivapi.com/InstanceContent/{}?snake_case=1&columns=ContentFinderCondition\
    .ClassJobLevelRequired,ContentFinderCondition.ClassJobLevelSync,ContentFinderCondition\
    .ContentMemberType,ContentFinderCondition.ContentType.ID,ContentFinderCondition.ContentType.Name\
    ,ContentFinderCondition.Name,ContentFinderCondition.ItemLevelRequired,ContentFinderCondition\
    .ItemLevelSync,ContentFinderCondition.ID", id);
    let resp: DutyResult = client.get(&url).send()?.json()?;
    Ok(resp.content_finder_condition)
}

pub fn duty_by_name(client: &reqwest::Client, name: &str) -> Result<DutyInfo, CommandError> {
    let search = content_search(client, name, "InstanceContent")?;
    if search.pagination.results_total > 1 {
        Err(CommandError(format!(
            "Search not specific enough, found {} matching results",
            search.pagination.results_total
        )))
    } else if search.results.is_empty() {
        Err(CommandError(
            "No matching character found, try again!".to_string(),
        ))
    } else {
        duty_by_id(client, search.results.first().expect("duty").id)
    }
}
