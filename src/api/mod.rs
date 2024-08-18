use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use reqwest::Error;

const SEARCH_API_URL: &str = "https://api.waqi.info/search/";

pub fn search_stations_by_city_name(
    city: &str,
    api_token: &str,
) -> Result<serde_json::Value, Error> {
    let response = Client::new()
        .get(SEARCH_API_URL)
        .query(&[("token", api_token), ("keyword", city)])
        .header(USER_AGENT, "Rust Weather CLI application")
        .send()
        .expect("200 OK response expected")
        .json::<serde_json::Value>()
        .expect("expected the body to be json");

    // async version
    // let response = client
    //     .get("https://api.waqi.info/search/")
    //     .query(&[("token", api_token), ("keyword", args.city)])
    //     .send()
    //     .expect("a successful request")
    //     .json::<serde_json::Value>()
    //     .expect("expected the body to be json");

    Ok(response)
}

pub fn fetch_station_data(uid: i32, api_token: &str) -> Result<serde_json::Value, Error> {
    let response = Client::new()
        .get(&format!("https://api.waqi.info/feed/@{}/", uid))
        .query(&[("token", api_token)])
        .header(USER_AGENT, "Rust Weather CLI application")
        .send()
        .expect("200 OK response expected")
        .json::<serde_json::Value>()
        .expect("expected the body to be json");

    Ok(response)
}
