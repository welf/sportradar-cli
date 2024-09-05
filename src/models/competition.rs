use std::fmt::Display;

use serde::Deserialize;

use crate::services::{BaseInfoService, CountryService};

use super::country::Country;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Competition {
    pub id: String,
    pub name: String,
    pub category: Country,
}

impl Display for Competition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.country_name())
    }
}

impl BaseInfoService for Competition {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl CountryService for Competition {
    fn country_name(&self) -> String {
        self.category.name()
    }
}
