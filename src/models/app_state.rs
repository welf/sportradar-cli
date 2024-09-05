use std::collections::{HashMap, HashSet};

use crate::{
    enums::Players,
    models::{Competition, CompetitionSeason, Player, PlayerStatistics, Sport, SportEvent, Team},
    services::{ApiService, AppStateService},
};

#[derive(Debug, Clone)]
pub struct AppState<HttpClient: ApiService> {
    client: HttpClient,                          // HTTP client with retry middleware
    sports: HashSet<Sport>,                      // Available sports (in trial mode only soccer)
    selected_sport: Option<Sport>, // User selected sport (in trial mode is set by default to soccer)
    competitions: HashSet<Competition>, // Available competitions in selected sport (and only those )
    selected_competition: Option<Competition>, // User selected competition
    seasons: Vec<CompetitionSeason>, // Available seasons in selected competition (disabled seasons are filtered out)
    selected_season: Option<CompetitionSeason>, // User selected season
    sport_events: HashSet<SportEvent>, // All sport events in selected competition and season
    competitors: HashSet<Team>,      // All competitors (teams) in selected competition and season
    players: HashMap<String, Player>, // All players in selected competition and season
    selected_player_statistics: Option<Players>, // User selected player statistics to show
    limit: Option<usize>,            // Limit to show the info to users
}

impl<HttpClient: ApiService> AppState<HttpClient> {
    pub fn new(client: HttpClient) -> Self {
        Self {
            client,
            sports: HashSet::new(),
            selected_sport: None,
            competitions: HashSet::new(),
            selected_competition: None,
            seasons: Vec::new(),
            selected_season: None,
            sport_events: HashSet::new(),
            competitors: HashSet::new(),
            players: HashMap::new(),
            selected_player_statistics: None,
            limit: None,
        }
    }
}

impl<HttpClient: ApiService>
    AppStateService<
        HttpClient,
        Sport,
        Competition,
        CompetitionSeason,
        SportEvent,
        Team,
        Player,
        PlayerStatistics,
    > for AppState<HttpClient>
{
    fn get_http_client(&self) -> &HttpClient {
        &self.client
    }

    fn sports(&self) -> HashSet<Sport> {
        self.sports.clone()
    }

    fn set_sports(&mut self, sports: HashSet<Sport>) {
        self.sports = sports;
    }

    fn selected_sport(&self) -> Option<Sport> {
        self.selected_sport.clone()
    }

    fn set_selected_sport(&mut self, sport: &Sport) {
        self.selected_sport = Some(sport.clone());
    }

    fn competitions(&self) -> HashSet<Competition> {
        self.competitions.clone()
    }

    fn set_competitions(&mut self, competitions: HashSet<Competition>) {
        self.competitions = competitions;
    }

    fn selected_competition(&self) -> Option<Competition> {
        self.selected_competition.clone()
    }

    fn set_selected_competition(&mut self, competition: &Competition) {
        self.selected_competition = Some(competition.clone());
    }

    fn seasons(&self) -> Vec<CompetitionSeason> {
        self.seasons.clone()
    }

    fn set_seasons(&mut self, seasons: Vec<CompetitionSeason>) {
        self.seasons = seasons;
    }

    fn selected_season(&self) -> Option<CompetitionSeason> {
        self.selected_season.clone()
    }

    fn set_selected_season(&mut self, season: CompetitionSeason) {
        self.selected_season = Some(season);
    }

    fn sport_events(&self) -> HashSet<SportEvent> {
        self.sport_events.clone()
    }

    fn set_sport_events(&mut self, sport_events: HashSet<SportEvent>) {
        self.sport_events = sport_events;
    }

    fn competitors(&self) -> HashSet<Team> {
        self.competitors.clone()
    }

    fn set_competitors(&mut self, competitors: HashSet<Team>) {
        self.competitors = competitors;
    }

    fn players(&self) -> HashMap<String, Player> {
        self.players.clone()
    }

    fn set_players(&mut self, players: HashMap<String, Player>) {
        self.players = players;
    }

    fn set_limit(&mut self, limit: usize) {
        self.limit = Some(limit);
    }

    fn limit(&self) -> Option<usize> {
        self.limit
    }

    fn selected_player_statistics(&self) -> Option<Players> {
        self.selected_player_statistics
    }

    fn set_selected_player_statistics(&mut self, players: Players) {
        self.selected_player_statistics = Some(players);
    }

    // Remove all the data except the http client, sports in trial mode, and competitions to let the user select a new competition
    fn reset(&mut self) {
        // Don't clear sports because we always fetch same sports during the session
        // self.sports.clear();
        self.selected_sport = None;
        self.competitions.clear();
        self.selected_competition = None;
        self.seasons.clear();
        self.selected_season = None;
        self.sport_events.clear();
        self.competitors.clear();
        self.players.clear();
        self.selected_player_statistics = None;
        self.limit = None;
    }
}
