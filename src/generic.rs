use serde::de::DeserializeOwned;

pub trait XIVApiObject: DeserializeOwned {
    type Id: Sized;
    fn endpoint(id: Self::Id) -> String;
    fn columns() -> &'static str;
    fn extra_params() -> Option<&'static str>;

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
    T: XIVGameContentObject + DeserializeOwned,
{
    type Id = usize;

    fn endpoint(id: usize) -> String {
        format!("/{}/{}", T::index(), id)
    }

    fn columns() -> &'static str {
        T::columns()
    }

    fn extra_params() -> Option<&'static str> {
        T::extra_params()
    }
}
