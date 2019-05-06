use serde::Deserialize;
use serde_repr::*;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RawCharacter {
    pub avatar: String,
    #[serde(alias = "ID")]
    pub id: usize,
    pub name: String,
    pub server: String
}

#[derive(Deserialize_repr, PartialEq, Debug, Display)]
#[repr(usize)]
pub enum Gender {
    Other = 0,
    Male = 1,
    Female = 2,
}

#[derive(Deserialize_repr, PartialEq, Debug, Display)]
#[repr(usize)]
pub enum Town {
    #[strum(to_string="Limsa Lominsa")]
    Limsa = 1,
    Gridania = 2,
    #[strum(to_string="Ul'dah")]
    Uldah = 3
}

#[derive(Deserialize_repr, PartialEq, Debug, Display)]
#[repr(usize)]
pub enum Race {
    Hyur = 1,
    Elezen = 2,
    Lalafell = 3,
    #[strum(to_string="Miqo'te")]
    Miqote = 4,
    Roegadyn = 5,
    #[strum(to_string="Au Ra")]
    Aura = 6,
}

#[derive(Deserialize_repr, PartialEq, Debug, Display)]
#[repr(usize)]
pub enum GuardianDeity {
    Halone = 1,
    Menphina = 2,
    Thaliak = 3,
    Nymeia = 4,
    Llymlaen = 5,
    Oschon = 6,
    Byregot = 7,
    Rhalgr = 8,
    Azeyma = 9,
    Naldthal = 10,
    Nophica = 11,
    Althyk = 12
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Character {
    pub avatar: String,
    #[serde(rename = "ID")]
    pub id: usize,
    pub guardian_deity: GuardianDeity,
    pub gender: Gender,
    pub portrait: String,
    pub race: Race,
    pub server: String,
    #[serde(rename = "FreeCompanyId")]
    pub fc: String,
    pub title: usize,
    pub town: Town,
    pub nameday: String,
    pub name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LodestoneIdResult {
    pub character: Character
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Pagination {
    pub results_total: usize
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LodestoneSearchResult {
    pub pagination: Pagination,
    pub results: Vec<RawCharacter>
}