use super::{BaseInfoService, PlayerStatisticsService};

pub trait PlayerService<T: PlayerStatisticsService, Competitor: BaseInfoService> {
    fn set_team(&mut self, team: &Competitor);
}
