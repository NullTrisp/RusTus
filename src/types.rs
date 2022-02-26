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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawStop {
    #[serde(rename = "ayto:parada")]
    pub name: String,
    #[serde(rename = "dc:modified")]
    modification_date: String,
    #[serde(rename = "wgs84_pos:lat")]
    pub wgs84_pos_lat: String,
    #[serde(rename = "wgs84_pos:long")]
    pub wgs84_pos_long: String,
    #[serde(rename = "ayto:numero")]
    pub number: String,
    #[serde(rename = "ayto:coordX_ETRS89")]
    coord_x_extrs89: String,
    #[serde(rename = "ayto:coordY_ETRS89")]
    coord_y_extrs89: String,
    #[serde(rename = "gn:coordX")]
    coord_x: String,
    #[serde(rename = "gn:coordY")]
    coord_y: String,
    #[serde(rename = "ayto:sentido")]
    pub direction: String,
    #[serde(rename = "dc:identifier")]
    pub id: String,
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stop {
    pub id: i32,
    pub direction: String,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub number: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseStops {
    pub summary: Summary,
    pub resources: Vec<RawStop>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Offest {
    pub from: i32,
    pub to: i32,
}
