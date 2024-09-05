use std::fmt::Display;

use serde::Deserialize;

use crate::services::{
    BaseInfoService, PlayerSeasonStatisticsService, PlayerService, PlayerStatisticsService,
};

use super::{PlayerStatistics, Team};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub statistics: PlayerStatistics,
    #[serde(default)]
    pub season_statistics: PlayerStatistics,
    #[serde(skip)]
    team: Team,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.team.name())
    }
}

impl BaseInfoService for Player {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl PlayerService<PlayerStatistics, Team> for Player {
    fn set_team(&mut self, team: &Team) {
        self.team = team.clone();
    }
}

impl PlayerStatisticsService for Player {
    fn goals(&self) -> u8 {
        self.statistics.goals()
    }

    fn assists(&self) -> u8 {
        self.statistics.assists()
    }
}

impl PlayerSeasonStatisticsService for Player {
    fn season_goals(&self) -> u8 {
        self.season_statistics.goals()
    }

    fn season_assists(&self) -> u8 {
        self.season_statistics.assists()
    }

    fn update_season_statistics(&mut self, player: &Self) {
        self.season_statistics.goals_scored += player.goals();
        self.season_statistics.assists += player.assists();
    }
}
