# Matchday Backend Developer Take Home Assignment

## Preface

The initial take-home assignment is described in the [Matchday_Rust_assignment_v2.md](Matchday_Rust_assignment_v2.md) file.

## Solution description

Current solution is much more larger than the one requested in the assignment. The reason for this is that I wanted to create a more complete solution that can be easily extended to other sports, competitions, and statistics, which can be achieved by switching to the paid version of the Sportradar API.

All the application logic is abstracted in the [`AppStateService`](src/services/app_state_service.rs) service trait. It is generic over all the structs ([`models`](src/models/) and [`API response DTOs`](src/api_responses/)) that are used in the application. All the behaviour of these structs needed by the application is abstracted in ([`services`](src/services/)) which allows for easy testing, mocking, or replacing with different implementations. Even HTTP client functionality used in the application is abstracted in the [`ApiService`](src/services/api_service.rs) service trait. Thus, the application is easy to extend, test, and refactor.

From the user perspective, the application is also much larger than the one requested in the assignment. The user can interactively choose the desired sport (which is only football (_soccer_) in the trial version of the API, therefore the data fetching for sports is mocked), the competition, the season, and the statistics to be displayed as well as the number of goal scorers or assistants to be displayed. After the desired statistics is displayed, the user is asked if they want to explore another sport, competition, season, or statistics. If the user chooses to proceed, the application state is reset and the user can choose again. Otherwise, the application exits with the nice message.

## Some notes

- Although I love to write tests, I did not write any tests for the application. The reason for this is that I wanted to focus on the application logic and its abstracted behaviour.
- You can use 2 different HTTP clients in the application:
  - [`reqwest::Client`](https://crates.io/crates/reqwest) - first, I used this client to fetch the data from the Sportradar API. But it seems the trial version of the API does not work well with a bunch of concurrent requests and often does not respond to some of the requests sent in batch. Therefore, I switched to the second client (but I left the implementation of the `ApiService` trait for the `reqwest::Client`, so you can pass the instance of the `reqwest::Client` to the `AppState` in the [`main`](src/main.rs) function if you want to use it);
  - [`reqwest_middleware::ClientWithMiddleware`](https://crates.io/crates/reqwest-retry) - this is a wrapper around the `reqwest::Client` that adds retry functionality to the client. Although the number of retries is set to 3, sometimes the client still fails to fetch the data from the Sportradar API and I leaved a debug message in the code to indicate what endpoint was not fetched and why.
- The application fetches all the competitions but allows the user to choose only those for which [the official statistics is available](https://developer.sportradar.com/soccer/reference/soccer-league-timeline). For the filtering `ALLOWED_COUNTRIES` and `ALLOWED_COMPETITIONS` environment variables are used.
- All the environment variables are loaded from the `.env` file. To run the application, you need to rename the `.env.example` file to `.env` and add your Sportradar API key to the `SPORT_RADAR_API_KEY` variable. All other variables from the `.env.example` file you can leave as they are.
- The project structure is relatively straitforward and you easily can understand what files are doing looking on file names and folder name where they are located.

## How to run the application

- Rename the `.env.example` file to `.env` and add your Sportradar API key to the `SPORT_RADAR_API_KEY` variable. All other variables from the `.env.example` file you can leave as they are.
- Run `cargo run` in the project root directory.
- Follow the instructions in the terminal.
