use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use itertools::Itertools;

use crate::{
    api_responses::{
        CompetitionSeasonsApiResponse, CompetitionsApiResponse, CompetitorStatisticsApiResponse,
        SchedulesApiResponse,
    },
    enums::Players,
    helpers::{
        get_allowed_competitions, get_allowed_countries, get_api_url, prompt_boolean,
        prompt_number, prompt_select,
    },
};

use super::{
    ApiService, BaseInfoService, CompetitorsService, ConstructService, CountryService, IdService,
    PlayerSeasonStatisticsService, PlayerService, PlayerStatisticsService, SeasonService,
};

/// The `AppStateService` trait defines the interface for managing the application state, including sports,
/// competitions, seasons, sport events, competitors, and players.
///
/// The trait is generic over almost all application state's types (except `limit`'s type which is `usize`),
/// including the HTTP client, sport, competition, season, sport event, competitor, player, and player
/// statistics. Each of these types must implement specific traits to be compatible with the `AppStateService`.
///
/// The trait provides methods for setting and retrieving the state of the application, as well as handling the
/// selection of sports, competitions, seasons, and player statistics. It also includes methods for fetching and
/// managing the data required for the application, such as competitors and players.
///
/// The `run()` method is the main entry point for the application, which guides the user through the process of
/// selecting a sport, competition, season, and player statistics to display.
pub trait AppStateService<
    HttpClient,
    Sport,
    Competition,
    Season,
    SportEvent,
    Competitor,
    Player,
    PlayerStatistics,
> where
    HttpClient: ApiService,
    Sport: BaseInfoService + ConstructService,
    Competition: BaseInfoService + CountryService,
    Season: BaseInfoService + SeasonService,
    SportEvent: CompetitorsService<Competitor> + IdService,
    Competitor: BaseInfoService + ConstructService,
    Player: BaseInfoService
        + PlayerStatisticsService
        + PlayerSeasonStatisticsService
        + PlayerService<PlayerStatistics, Competitor>,
    PlayerStatistics: PlayerStatisticsService,
{
    // ==================== METHODS TO IMPLEMENT IN IMPLS ====================

    // Get the HTTP client
    fn get_http_client(&self) -> &HttpClient;

    fn set_sports(&mut self, sports: HashSet<Sport>);

    // Get available sports
    fn sports(&self) -> HashSet<Sport>;

    // Set selected sport
    fn set_selected_sport(&mut self, sport: &Sport);

    // Get the selected sport ("soccer" in trial version)
    fn selected_sport(&self) -> Option<Sport>;

    // Set fetched and filtered competitions
    fn set_competitions(&mut self, competitions: HashSet<Competition>);

    // Get available competitions
    fn competitions(&self) -> HashSet<Competition>;

    // Set selected competition
    fn set_selected_competition(&mut self, competition: &Competition);

    // Get selected competition
    fn selected_competition(&self) -> Option<Competition>;

    // Set fetched and filtered seasons
    fn set_seasons(&mut self, seasons: Vec<Season>);

    // Get available seasons
    fn seasons(&self) -> Vec<Season>;

    // Set selected Season
    fn set_selected_season(&mut self, season: Season);

    // Get selected season (if any)
    fn selected_season(&self) -> Option<Season>;

    // Set sports events
    fn set_sport_events(&mut self, events: HashSet<SportEvent>);

    // Get sports events
    fn sport_events(&self) -> HashSet<SportEvent>;

    // Set competitors and fetch
    fn set_competitors(&mut self, competitors: HashSet<Competitor>);

    // Get competitors
    fn competitors(&self) -> HashSet<Competitor>;

    // Set players
    fn set_players(&mut self, players: HashMap<String, Player>);

    // Get players
    fn players(&self) -> HashMap<String, Player>;

    fn selected_player_statistics(&self) -> Option<Players>;

    fn set_selected_player_statistics(&mut self, players: Players);

    // Set the limit for the number of players to display
    fn set_limit(&mut self, limit: usize);

    // Get the limit for the number of players to display
    fn limit(&self) -> Option<usize>;

    fn reset(&mut self);

    // ==================== METHODS FOR FREE ====================

    /// The `on_start` method is called when the application is started.
    /// It constructs a default `Sport` instance and returns it as a `HashSet` of sports.
    /// The fetching of sports is mocked in this example.
    async fn on_start(&mut self) -> Result<HashSet<Sport>, Box<dyn Error>> {
        let sports = if self.sports().is_empty() {
            // Mock fetching sports
            let mut set = HashSet::<Sport>::new();
            let sport = Sport::construct("sr:sport:1".to_string(), "Soccer".to_string());
            set.insert(sport);
            set
        } else {
            self.sports()
        };

        Ok(sports)
    }

    /// The `on_sport_select` method is called when a sport is selected.
    /// It fetches the list of competitions for the selected sport, filters them
    /// based on allowed competitions and countries, and sets the fetched and filtered
    /// competitions in the application state.
    ///
    /// This method performs the following steps:
    ///
    /// 1. Retrieves the selected sport from the application state.
    /// 2. Sets the selected sport in the application state.
    /// 3. Constructs the API URL for fetching the competitions.
    /// 4. Retrieves the HTTP client from the application state.
    /// 5. Fetches the list of allowed competitions and countries from the environment variables.
    /// 6. Fetches the list of competitions for the selected sport from the API.
    /// 7. Filters the competitions based on the allowed competitions and countries.
    /// 8. Sets the filtered competitions in the application state.
    ///
    /// # Errors
    ///
    /// This method can return an error if there is a problem fetching the competitions from the API
    /// or setting the competitions in the application state.
    async fn on_sport_select(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sport = self
            .selected_sport()
            .expect("No selected sport to fetch competitions");

        self.set_selected_sport(&sport);
        let api_url = get_api_url(&sport, "competitions");
        let client = self.get_http_client();

        // Get the list of allowed competitions and countries from the environment variables
        let allowed_competitions = get_allowed_competitions();
        let allowed_countries = get_allowed_countries();

        // Fetch the competitions for the selected sport
        let response = client
            .get_json_data::<CompetitionsApiResponse<Competition>>(api_url)
            .await?;

        let competitions = response
            .competitions
            .into_iter()
            // Filter out competitions for which the `league_timeline` stats are not available
            .filter(|competition: &Competition| {
                allowed_competitions.contains(&competition.name())
                    && allowed_countries.contains(&competition.country_name())
            })
            .collect();

        // Set competitions to the state
        self.set_competitions(competitions);
        Ok(())
    }

    /// This method is called after a competition was selected by the user. It fetches the seasons
    /// for the selected competition from the API and sets them in the application state.
    ///
    /// # Errors
    ///
    /// This method can return an error if there is a problem fetching the seasons from the API or
    /// setting the seasons in the application state.
    async fn on_competition_select(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sport = self
            .selected_sport()
            .expect("No selected sport to fetch seasons");
        let competition = self
            .selected_competition()
            .expect("No selected competition to fetch seasons");

        let client = self.get_http_client();
        let api_url = get_api_url(&sport, format!("competitions/{}/seasons", competition.id()));

        // Fetch the seasons for the selected competition
        let response = client
            .get_json_data::<CompetitionSeasonsApiResponse<Season>>(api_url)
            .await?;

        // Filter out disabled seasons
        let seasons = response
            .seasons
            .into_iter()
            .filter(|season| season.is_enabled())
            .sorted_by(|a, b| Ord::cmp(&b.name(), &a.name()))
            .collect();

        // Set fetched and filtered seasons
        self.set_seasons(seasons);
        Ok(())
    }

    /// This method is called after a season was selected by the user. It fetches the sport events
    /// and competitors for the selected season from the API and sets them in the application state.
    /// It also fetches the players and sets them in the application state.
    ///
    /// # Errors
    ///
    /// This method can return an error if there is a problem fetching the sport events, competitors,
    /// or players from the API or setting them in the application state.
    async fn on_season_select(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sport = self
            .selected_sport()
            .expect("No selected sport to fetch sport events");
        let season = self
            .selected_season()
            .expect("No selected season to fetch sport events");

        let api_url = get_api_url(&sport, format!("seasons/{}/schedules", season.id()));
        let client = self.get_http_client();

        // Fetch the sport events for the selected season
        let response = client
            .get_json_data::<SchedulesApiResponse<SportEvent>>(api_url)
            .await
            .expect("Failed to fetch sport events");

        // Extract the sport events from the response
        let sport_events: HashSet<SportEvent> = response
            .schedules
            .into_iter()
            .map(|schedule| schedule.sport_event)
            .collect();

        // Get the list of competitors from the sport_events
        let competitors: HashSet<Competitor> = sport_events
            .iter()
            .flat_map(|event| event.competitors())
            .collect();

        // Set sports events and competitors to the state
        self.set_sport_events(sport_events);
        self.set_competitors(competitors);

        // Fetch players
        let players = self.fetch_players().await?;

        // Set players to the state
        self.set_players(players);

        Ok(())
    }

    /// Returns a set of API URLs for fetching competitor statistics for the currently selected sport
    /// and season.
    ///
    /// This method iterates over the set of competitors in the application state, and generates an API
    /// URL for each competitor to fetch their statistics for the currently selected season. The API URLs
    /// are constructed using the `get_api_url` function, which takes the selected sport and a path that
    /// includes the season ID and competitor ID.
    ///
    /// # Returns
    /// A `HashSet<String>` containing the API URLs for fetching competitor statistics.
    fn competitors_api_urls(&self) -> HashSet<String> {
        let sport = self
            .selected_sport()
            .expect("No selected sport to fetch competitors");
        let season = self
            .selected_season()
            .expect("No selected season to fetch competitors");
        let competitors = self.competitors();

        competitors
            .iter()
            .map(|competitor: &Competitor| {
                get_api_url(
                    &sport,
                    format!(
                        "seasons/{}/competitors/{}/statistics",
                        season.id(),
                        competitor.id()
                    ),
                )
            })
            .collect()
    }

    /// Handles the response from the competitor statistics API, updating the players map with the data
    /// from the response.
    ///
    /// This method is responsible for processing the response from the competitor statistics API.
    /// It extracts the competitor and player data from the response, and then updates the players
    /// map accordingly. If a player already exists in the map, their season statistics are updated.
    /// If a player is new, they are added to the map.
    ///
    /// # Arguments
    ///
    /// * `players` - A mutable reference to the players map, which will be updated with the data
    ///     from the API response.
    /// * `response` - The result of the API request, containing the competitor and player data.
    fn handle_competitor_statistics_response(
        &self,
        players: &mut HashMap<String, Player>,
        response: Result<CompetitorStatisticsApiResponse<Player>, Box<dyn Error>>,
    ) {
        match response {
            Ok(response) => {
                // Create a new Competitor instance from the response data
                let team = Competitor::construct(response.competitor.id, response.competitor.name);

                for mut player in response.competitor.players {
                    // Set the team for the player
                    player.set_team(&team);

                    let player_id = player.id();
                    let updated_player = match players.get_mut(&player_id) {
                        // If the player already exists in the map
                        Some(existing_player) => {
                            // Update the existing player's season statistics
                            existing_player.update_season_statistics(&player);
                            // Clone the updated player to insert it back into the map
                            existing_player.clone()
                        }
                        // If the player is new
                        None => {
                            // Clone the player and update its season statistics
                            let mut new_player = player.clone();
                            new_player.update_season_statistics(&player);
                            new_player
                        }
                    };

                    // Insert or update the player in the map
                    players.insert(player_id, updated_player);
                }
            }
            Err(err) => {
                eprintln!("Error fetching competitor statistics: {}", err);
            }
        }
    }

    /// Fetches the player statistics for all competitors and updates the internal player map.
    ///
    /// This method is responsible for fetching the player statistics for all competitors from the API,
    /// and then updating the internal player map accordingly. If a player already exists in the map,
    /// their season statistics are updated. If a player is new, they are added to the map.
    ///
    /// # Returns
    /// A `Result` containing a `HashMap` of `Player` objects, keyed by their unique identifier.
    async fn fetch_players(
        &mut self,
    ) -> Result<HashMap<String, Player>, Box<dyn std::error::Error>> {
        let client = self.get_http_client();

        // Create a vector of API URLs to fetch the statistics for each competitor
        let api_urls = self.competitors_api_urls();

        let mut players = HashMap::new();

        // Fetch the statistics for each competitor concurrently using join_all
        futures::future::join_all(
            api_urls
                .into_iter()
                .map(|url| client.get_json_data::<CompetitorStatisticsApiResponse<Player>>(url)),
        )
        .await
        .into_iter()
        // Handle the response for each competitor
        .for_each(|response| self.handle_competitor_statistics_response(&mut players, response));

        Ok(players)
    }

    /// Gets the top goal scorers sorted in descending order.
    ///
    /// This method returns a vector of the top goal scorers, sorted in descending order by the number
    /// of goals scored in the current season. The number of players returned is limited by the `limit()`
    /// method, which defaults to 10 if not set.
    ///
    /// # Returns
    /// A vector of `Player` objects representing the top goal scorers.
    fn get_top_goal_scorers(&self) -> Vec<Player> {
        self.players()
            .clone()
            .into_values()
            .sorted_by(|a, b| Ord::cmp(&b.season_goals(), &a.season_goals()))
            .take(self.limit().unwrap_or(10))
            .collect()
    }

    /// Gets the top assistants sorted in descending order.
    ///
    /// This method returns a vector of the top assistants, sorted in descending order by the number of
    /// assists in the current season. The number of players returned is limited by the `limit()` method,
    /// which defaults to 10 if not set.
    ///
    /// # Returns
    /// A vector of `Player` objects representing the top assistants.
    fn get_top_assistants(&self) -> Vec<Player> {
        self.players()
            .clone()
            .into_values()
            .sorted_by(|a, b| Ord::cmp(&b.season_assists(), &a.season_assists()))
            .take(self.limit().unwrap_or(10))
            .collect()
    }

    /// Runs the application, allowing the user to select a sport, competition, season, and player
    /// statistics to view.
    ///
    /// This method is the main entry point for the application. It guides the user through the process
    /// of selecting a sport, competition, season, and player statistics to view. If any of these are
    /// not selected, the method prompts the user to make a selection.
    ///
    /// Once all selections are made, the method displays the top goal scorers or top assistants for the
    /// selected season, based on the user's choice. The number of players displayed is determined by the
    /// limit set by the user.
    ///
    /// The method runs in a loop, allowing the user to explore other competitions or seasons.
    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            // Reset the state
            self.reset();

            // If the sport is not selected let the user select it
            if self.selected_sport().is_none() {
                let sports = self.on_start().await?; // fetch sports and save them to the state
                let options: Vec<&Sport> = sports.iter().collect(); // get the sports
                let selected_sport = prompt_select("Select a sport:", options, 15)?;
                self.set_selected_sport(selected_sport); // set the selected sport
            }

            // If the competition is not selected, let the user select it
            if self.selected_competition().is_none() {
                self.on_sport_select().await?; // fetch competitions and save them to the state
                let competitions = self.competitions(); // get the competitions
                let options: Vec<Competition> = competitions.into_iter().collect(); // convert to a select options
                let selected_competition = prompt_select("Select a competition:", options, 15)?;
                self.set_selected_competition(&selected_competition); // set the selected competition
            }

            // If the season is not selected, let the user select it
            if self.selected_season().is_none() {
                self.on_competition_select().await?; // fetch seasons and save them to the state
                let seasons = self.seasons(); // get the seasons
                let options: Vec<Season> = seasons.into_iter().collect(); // convert to a select options
                let selected_season = prompt_select("Select a season:", options, 15)?;
                self.set_selected_season(selected_season); // set the selected season
            }

            // If the player statistics are not selected, let the user select them
            if self.selected_player_statistics().is_none() {
                self.on_season_select().await?; // fetch players and save them to the state
                let options: Vec<Players> = vec![Players::TopGoalScorers, Players::TopAssistants]; // player statistics options
                let selected_player_statistics =
                    prompt_select("What statistics do you want to see?", options, 15)?;
                self.set_selected_player_statistics(selected_player_statistics);
                // set the selected player statistics
            }

            // If the limit is not set, let the user set it
            if self.limit().is_none() {
                let limit = prompt_number("How many players do you want to see?")?; // user choice for the limit
                self.set_limit(limit); // set the limit
            }

            // Print the players with their statistics depending on the selected player statistics
            if let Some(option) = self.selected_player_statistics() {
                match option {
                    Players::TopGoalScorers => {
                        let top_goal_scorers = self.get_top_goal_scorers();
                        for (index, player) in top_goal_scorers.iter().enumerate() {
                            println!(
                                "{}. {} - {} goals",
                                index + 1,
                                player,
                                player.season_goals()
                            );
                        }
                    }
                    Players::TopAssistants => {
                        let top_assistants = self.get_top_assistants();
                        for (index, player) in top_assistants.iter().enumerate() {
                            println!(
                                "{}. {} - {} assists",
                                index + 1,
                                player,
                                player.season_assists()
                            );
                        }
                    }
                }
            }

            // Ask the users if they want to continue
            prompt_boolean("Do you want to explore other sports, competitions, or seasons?");
        }
        Ok(())
    }
}
