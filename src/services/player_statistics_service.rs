use serde::de::DeserializeOwned;
use std::hash::Hash;

pub trait PlayerStatisticsService
where
    Self: DeserializeOwned + Clone + PartialEq + Eq + Hash,
{
    fn goals(&self) -> u8;
    fn assists(&self) -> u8;
}
