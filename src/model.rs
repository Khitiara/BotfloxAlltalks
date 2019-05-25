use serde::Deserialize;
use serde_repr::*;
use strum_macros::Display;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct RawCharacter {
    pub avatar: String,
    pub id: usize,
    pub name: String,
    pub server: String,
}

#[derive(Deserialize_repr, PartialEq, Debug, Display)]
#[repr(usize)]
pub enum Gender {
    Other = 0,
    Male = 1,
    Female = 2,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct WithIdName {
    pub id: usize,
    pub name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ActiveClassJob {
    pub class: WithIdName,
    pub level: usize,
    pub job: WithIdName,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Character {
    pub active_class_job: ActiveClassJob,
    pub avatar: String,
    pub id: usize,
    pub guardian_deity: WithIdName,
    pub gender: Gender,
    pub portrait: String,
    pub race: WithIdName,
    pub server: String,
    pub free_company_id: Option<String>,
    pub title: WithIdName,
    pub town: WithIdName,
    pub tribe: WithIdName,
    pub nameday: String,
    pub name: String,
    #[serde(skip_deserializing)]
    pub fc: FreeCompany,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct LodestoneCharacterIdResult {
    pub character: Character,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Pagination {
    pub results_total: usize,
    pub page: usize,
    pub page_next: Option<usize>,
    pub page_prev: Option<usize>,
    pub page_total: usize,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct LodestoneSearchResult<T> {
    pub pagination: Pagination,
    pub results: Vec<T>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct FreeCompany {
    pub name: String,
    pub id: String,
    pub tag: String,
}

impl Default for FreeCompany {
    fn default() -> Self {
        FreeCompany {
            name: "".to_string(),
            id: "".to_string(),
            tag: "".to_string(),
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct LodestoneFCIdResult {
    pub free_company: FreeCompany,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct PartyComp {
    pub healers_per_party: usize,
    pub melees_per_party: usize,
    pub ranged_per_party: usize,
    pub tanks_per_party: usize,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct DutyInfo {
    pub id: usize,
    pub class_job_level_sync: usize,
    pub class_job_level_required: usize,
    pub item_level_sync: usize,
    pub itel_level_required: usize,
    pub content_type: WithIdName,
    pub content_member_type: PartyComp,
    pub name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct DutyResult {
    pub content_finder_condition: DutyInfo,
}
