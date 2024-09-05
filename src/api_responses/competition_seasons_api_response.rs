use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CompetitionSeasonsApiResponse<Season> {
    pub seasons: Vec<Season>,
}
