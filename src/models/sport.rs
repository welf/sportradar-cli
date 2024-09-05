use std::fmt::Display;

use serde::Deserialize;

use crate::services::{BaseInfoService, ConstructService};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Sport {
    pub id: String,
    pub name: String,
}

impl Display for Sport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl BaseInfoService for Sport {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn name(&self) -> String {
        self.name.clone().to_lowercase()
    }
}

impl ConstructService for Sport {
    fn construct(id: String, name: String) -> Self {
        Self { id, name }
    }
}
