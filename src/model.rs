use serde::Deserialize;
use serde_repr::*;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct RawCharacter {
    avatar: String,
    #[serde(alias = "ID")]
    id: u32,
    name: String,
    server: String
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(usize)]
enum Gender {
    Other = 0,
    Male = 1,
    Female = 2,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(usize)]
enum Town {
    Limsa = 1,
    Gridania = 2,
    Uldah = 3
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct Character {
    avatar: String,
    #[serde(rename = "ID")]
    id: u32,
    guardian_diety: u8,
    gender: Gender,
    portrait: String,
    race: u8,
    server: String,
    #[serde(rename = "FreeCompanyId")]
    fc: u32,
    title: u16,
    town: Town
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct LodestoneIdResult {
    character: Character
}