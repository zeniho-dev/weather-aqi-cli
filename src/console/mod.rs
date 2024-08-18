use crate::models;
use std::io::{stdin, stdout, Write};

pub fn init() {
    // Clear the console screen
    println!("{esc}c", esc = 27 as char);
    println!("Welcome to the AQI console Provider (Press Ctrl+C to exit)");
}

pub fn ask_for_city() -> String {
    print!("Search a station by city name $ ");
    stdout().flush().unwrap();
    let mut city = String::new();
    stdin().read_line(&mut city).unwrap();
    city.trim().to_string()
}

pub fn ask_for_station(data: &Vec<models::StationData>) -> &models::StationData {
    println!("Choose a station:");
    for (index, station_data) in data.iter().enumerate() {
        println!("{}. {}", index + 1, station_data.station.name);
    }

    print!("Enter the station number $ ");
    stdout().flush().unwrap();
    let mut station_number = String::new();
    stdin().read_line(&mut station_number).unwrap();
    let station_number: usize = station_number.trim().parse().unwrap();

    &data[station_number - 1]
}
