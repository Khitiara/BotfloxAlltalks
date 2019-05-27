use serde::de::DeserializeOwned;

pub trait XIVApiObject: DeserializeOwned {
    type Id: Sized;
    fn id_url(s: Self::Id) -> String;
}
