use crate::model::WithIdName;
use serde::de::DeserializeOwned;

pub trait XIVApiObject {
    type Search;
    type SearchResult: DeserializeOwned;
    type Id: Sized;
    type IdResult: DeserializeOwned;
    fn endpoint(id: Self::Id) -> String;
    fn search_endpoint(s: Self::Search) -> String;
    fn columns() -> &'static str;
    fn extra_params() -> Option<&'static str>;
    fn id_from_search_result(s: &Self::SearchResult) -> Self::Id;

    fn search_url(s: Self::Search) -> String {
        format!(
            "https://xivapi.net{}&snake_case=1",
            Self::search_endpoint(s)
        )
    }

    fn id_url(s: Self::Id) -> String {
        let parms = Self::extra_params()
            .map(|s| format!("&{}", s))
            .unwrap_or_default();
        format!(
            "https://xivapi.net{}&snake_case=1{}&columns={}",
            Self::endpoint(s),
            parms,
            Self::columns()
        )
    }
}

pub trait XIVGameContentObject {
    fn index() -> &'static str;
    fn columns() -> &'static str;
    fn extra_params() -> Option<&'static str>;
}

impl<'de, T> XIVApiObject for T
where
    T: XIVGameContentObject,
    T: DeserializeOwned,
{
    type Search = &'static str;
    type SearchResult = WithIdName;
    type Id = usize;
    type IdResult = T;

    fn endpoint(id: usize) -> String {
        format!("/{}/{}", T::index(), id)
    }

    fn search_endpoint(s: &'static str) -> String {
        format!("/search?indexes={}&string={}", T::index(), s)
    }

    fn columns() -> &'static str {
        T::columns()
    }

    fn extra_params() -> Option<&'static str> {
        T::extra_params()
    }

    fn id_from_search_result(s: &WithIdName) -> Self::Id {
        s.id
    }
}
