use serde::de::DeserializeOwned;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

pub trait BaseInfoService
where
    Self: DeserializeOwned + Clone + PartialEq + Eq + Hash + Display + Debug,
{
    fn id(&self) -> String;
    fn name(&self) -> String;
}
