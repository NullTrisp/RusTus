use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawBus {
    #[serde(rename = "ayto:numero")]
    pub number: String,
    #[serde(rename = "dc:name")]
    pub name: String,
    #[serde(rename = "dc:identifier")]
    pub id: String,
    uri: String,
}

impl fmt::Display for RawBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.number, self.name, self.id, self.uri)
    }
}

#[derive(Serialize)]
pub struct Destinations {
    pub a: Vec<String>,
    pub b: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct Bus {
    pub number: String,
    pub destinations: Destinations,
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Summary {
    items: i32,
    items_per_page: i32,
    pages: i32,
    current_page: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseBuses {
    pub summary: Summary,
    pub resources: Vec<RawBus>,
}
