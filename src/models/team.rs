use std::fmt::Display;

use serde::Deserialize;

use crate::services::{BaseInfoService, ConstructService};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct Team {
    pub id: String,
    pub name: String,
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl BaseInfoService for Team {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl ConstructService for Team {
    fn construct(id: String, name: String) -> Self {
        Self { id, name }
    }
}
