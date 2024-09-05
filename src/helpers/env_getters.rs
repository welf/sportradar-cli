use crate::services::BaseInfoService;

fn get_api_base_url() -> String {
    std::env::var("SPORT_RADAR_API_BASE_URL").expect("API url variable not found")
}

fn get_api_key() -> String {
    std::env::var("SPORT_RADAR_API_KEY").expect("API key not found")
}

fn get_access_level() -> String {
    std::env::var("SPORT_RADAR_API_ACCESS_LEVEL").expect("API access level variable not found")
}

fn get_language_code() -> String {
    std::env::var("SPORT_RADAR_API_LANGUAGE_CODE").unwrap_or_else(|_| "en".to_string())
}

fn get_format() -> String {
    std::env::var("SPORT_RADAR_API_FORMAT").unwrap_or_else(|_| "json".to_string())
}

pub fn get_allowed_countries() -> Vec<String> {
    std::env::var("ALLOWED_COUNTRIES")
        .unwrap_or_else(|_| "England,Germany,Italy,Spain,USA,Austria".into())
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

// Get the list of allowed competitions
pub fn get_allowed_competitions() -> Vec<String> {
    std::env::var("ALLOWED_COMPETITIONS")
        .unwrap_or_else(|_| {
            "Premier League,Bundesliga,Serie A,LaLiga,UEFA Champions League,MLS,Bundesliga".into()
        })
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

pub fn get_api_url<T: BaseInfoService>(sport: &T, endpoint: impl Into<String>) -> String {
    let sport = sport.name();

    format!(
        "{}/{}/{}/v4/{}/{}.{}?api_key={}",
        get_api_base_url(),
        sport,
        get_access_level(),
        get_language_code(),
        endpoint.into(),
        get_format(),
        get_api_key()
    )
}
