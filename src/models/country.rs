use std::fmt::Display;

use serde::Deserialize;

use crate::services::BaseInfoService;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Country {
    pub id: String,
    pub name: String,
}

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl BaseInfoService for Country {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}
