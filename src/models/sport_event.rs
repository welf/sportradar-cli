use serde::Deserialize;

use crate::services::CompetitorsService;

use super::Team;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct SportEvent {
    pub id: String,
    competitors: Vec<Team>,
}

impl CompetitorsService<Team> for SportEvent {
    fn competitors(&self) -> Vec<Team> {
        self.competitors.clone()
    }
}
