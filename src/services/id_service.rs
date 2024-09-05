use serde::de::DeserializeOwned;
use std::hash::Hash;

pub trait IdService
where
    Self: DeserializeOwned + Clone + PartialEq + Eq + Hash,
{
    fn id(&self) -> String;
}
