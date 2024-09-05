use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SchedulesApiResponse<Event> {
    pub schedules: Vec<Schedule<Event>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Schedule<Event> {
    pub sport_event: Event,
}
