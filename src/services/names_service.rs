use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub trait NamesService<T: Display + Clone, Collection: FromIterator<std::string::String>> {
    fn get_names(&self) -> Collection {
        self.get_items()
            .iter()
            .map(|item| item.to_string())
            .collect()
    }

    fn get_items(&self) -> Vec<T>;
}

impl<T: Display + Clone, Collection: FromIterator<String>> NamesService<T, Collection>
    for HashSet<T>
{
    fn get_items(&self) -> Vec<T> {
        self.iter().cloned().collect()
    }
}

impl<T: Display + Clone, Collection: FromIterator<String>> NamesService<T, Collection> for Vec<T> {
    fn get_items(&self) -> Vec<T> {
        self.clone()
    }
}

impl<T: Display + Clone, Collection: FromIterator<String>> NamesService<T, Collection>
    for HashMap<String, T>
{
    fn get_items(&self) -> Vec<T> {
        self.values().cloned().collect()
    }
}
