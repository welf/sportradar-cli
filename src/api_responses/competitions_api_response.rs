use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CompetitionsApiResponse<Competition> {
    pub competitions: Vec<Competition>,
}
