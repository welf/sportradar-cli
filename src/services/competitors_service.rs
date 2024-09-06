use serde::de::DeserializeOwned;
use std::hash::Hash;

use super::BaseInfoService;

pub trait CompetitorsService<T>
where
    T: BaseInfoService + DeserializeOwned + Clone + PartialEq + Eq + Hash,
{
    fn competitors(&self) -> Vec<T>;
}
