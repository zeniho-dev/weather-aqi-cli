use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub data: Vec<StationData>,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct StationData {
    pub aqi: i32,
    pub station: StationInfo,
    pub time: TimeInfo,
    pub uid: i32,
}

#[derive(Deserialize, Debug)]
pub struct StationInfo {
    pub country: Option<String>,
    pub geo: Vec<f64>,
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct TimeInfo {
    pub stime: String,
    pub tz: String,
    pub vtime: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum IaqiType {
    Dew(IaqiValue),
    H(IaqiValue),
    No2(IaqiValue),
    O3(IaqiValue),
    P(IaqiValue),
    Pm10(IaqiValue),
    Pm25(IaqiValue),
    T(IaqiValue),
    W(IaqiValue),
}

#[derive(Deserialize, Debug)]
pub struct IaqiValue {
    pub v: f64,
}

#[derive(Deserialize, Debug)]
pub struct Iaqi {
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, IaqiType>,
}

#[derive(Deserialize, Debug)]
pub struct StationFeed {
    pub status: String,
    pub data: StationFeedData,
}

#[derive(Deserialize, Debug)]
pub struct DebugInfo {
    pub sync: String,
}

#[derive(Deserialize, Debug)]
pub struct StationFeedData {
    pub idx: i32,
    pub aqi: i32,
    pub time: TimeInfo,
    pub city: StationInfo,
    pub debug: DebugInfo,
    pub dominentpol: String,
    //pub forecast: Forecast,
    pub iaqi: Iaqi,
}
