use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CompetitorStatisticsApiResponse<Player> {
    pub competitor: Competitor<Player>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Competitor<Player> {
    pub id: String,
    pub name: String,
    pub players: Vec<Player>,
}
