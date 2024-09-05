use serde::Deserialize;

use crate::services::PlayerStatisticsService;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct PlayerStatistics {
    pub assists: u8,
    pub goals_scored: u8,
}

impl PlayerStatisticsService for PlayerStatistics {
    fn goals(&self) -> u8 {
        self.goals_scored
    }
    fn assists(&self) -> u8 {
        self.assists
    }
}
