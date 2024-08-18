/*
* https://aqicn.org/data-platform/token-confirm/1ae903d3-c812-4e7f-a23c-660d52962e92
* Your token is d26a89688a7b04e53f43bcc71ae209a6374130ad
*/

mod api;
mod console;
mod models;

use clap::Parser;
use dotenv::dotenv;
use std::env;

#[derive(Parser, Debug)]
struct Args {
    // City name
    #[arg(short, long)]
    city: String,
}

fn main() {
    // Load the .env file
    dotenv().ok();

    let api_token = env::var("AQI_PROVIDER_API_TOKEN").expect("API_TOKEN expected");
    //let args = Args::parse();

    // Let's build an app that fetches the air quality index of a city
    // The user will provide the city name in console

    // 1. Init console app
    console::init();

    // 2. Ask for the city name
    // Later we'll ask for other API options like:
    // - Choose the AQI provider
    // - Choose the datas to fetch by city or coordinates
    let city_name = console::ask_for_city();

    let response = api::search_stations_by_city_name(&city_name, &api_token)
        .expect("expected a successful request");
    // let client = reqwest::blocking::Client::new();
    // let response = client
    //     .get("https://api.waqi.info/search/")
    //     .query(&[("token", api_token), ("keyword", args.city)])
    //     .send()
    //     .expect("a successful request")
    //     .json::<serde_json::Value>()
    //     .expect("expected the body to be json");

    // Transform the response into a struct
    let api_response: models::ApiResponse = serde_json::from_value(response).unwrap();

    // Read the data lenght : &api_response.data.len()
    // Read the first station name : &api_response.data[0].station.name

    // Iterate over the stations and print the name
    // for station_data in api_response.data {
    //     println!("Station name: {}", station_data.station.name);
    // }

    // Ask the user to choose a station
    let chosen_station = console::ask_for_station(&api_response.data);
    println!("You chose the station: {}", chosen_station.station.name);
    println!("UID: {}", chosen_station.uid);
    println!("AQI: {}", chosen_station.aqi);
    // Fetch the AQI data for the chosen station
    let station_data = api::fetch_station_data(chosen_station.uid, &api_token)
        .expect("expected a successful request");

    dbg!(&station_data);
}
