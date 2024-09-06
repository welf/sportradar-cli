use std::error::Error;

use dotenvy::dotenv;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

use self::{models::AppState, services::AppStateService};

mod api_responses;
mod enums;
mod helpers;
mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // ===== APP STATE SETUP =====

    // Create a new HTTP client with retry middleware to handle transient errors
    // Retry up to 3 times with increasing intervals between attempts.
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    // Create an instance of the app state
    let mut app: AppState<ClientWithMiddleware> = AppState::new(client);

    // ===== RUN THE APP =====

    app.run().await?;

    Ok(())
}
