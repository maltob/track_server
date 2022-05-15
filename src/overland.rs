use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OverlandMessage {
    pub locations: Vec<OverlandLocation>,
    pub current: Option<OverlandLocation>,
    trip: Option<OverlandTrip>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OverlandLocation {
    r#type: String,
    pub geometry: OverlandGeometry,
    pub properties: OverlandProperties,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OverlandGeometry {
    r#type: String,
    pub coordinates: Vec<f64>,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct OverlandProperties {
    timestamp: String,
    altitude: Option<i32>,
    speed: Option<i32>,
    horizontal_accuracy: Option<i32>,
    vertical_accuracy: Option<i32>,
    motion: Option<Vec<String>>,
    device_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OverlandTrip {
    distance: i64,
    mode: String,
    current_location: OverlandLocation,
    start_location: OverlandLocation,
    start: String,
}

#[derive(Serialize)]
pub struct OverlandResult {
    pub result: String,
}