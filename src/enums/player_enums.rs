use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Players {
    TopGoalScorers,
    TopAssistants,
}

impl Display for Players {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Players::TopGoalScorers => write!(f, "Top Goal Scorers"),
            Players::TopAssistants => write!(f, "Top Assistants"),
        }
    }
}
