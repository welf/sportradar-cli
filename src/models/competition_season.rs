use std::fmt::Display;

use serde::Deserialize;

use crate::services::{BaseInfoService, SeasonService};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct CompetitionSeason {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub disabled: bool,
}

impl Display for CompetitionSeason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl BaseInfoService for CompetitionSeason {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl SeasonService for CompetitionSeason {
    fn is_enabled(&self) -> bool {
        !self.disabled
    }
}
