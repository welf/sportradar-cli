use serde::de::DeserializeOwned;
use std::hash::Hash;

use super::PlayerStatisticsService;

pub trait PlayerSeasonStatisticsService
where
    Self: DeserializeOwned + Clone + PartialEq + Eq + Hash + PlayerStatisticsService,
{
    fn season_goals(&self) -> u8;

    fn season_assists(&self) -> u8;

    fn update_season_statistics(&mut self, player_statistics: &Self);
}
