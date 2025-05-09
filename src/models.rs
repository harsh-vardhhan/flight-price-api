use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flight {
    pub uuid: Uuid,
    pub date: NaiveDate,
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub time: String,
    pub duration: String,
    pub flight_type: String,
    pub price_inr: u32,
    pub origin_country: String,
    pub destination_country: String,
    pub rain_probability: u8,
}

// Rename fields from snake_case in JSON to camelCase in Rust struct
impl Flight {
    fn from_json(json: &serde_json::Value) -> Option<Self> {
        Some(Flight {
            uuid: serde_json::from_value(json["uuid"].clone()).ok()?,
            date: serde_json::from_value(json["date"].clone()).ok()?,
            origin: serde_json::from_value(json["origin"].clone()).ok()?,
            destination: serde_json::from_value(json["destination"].clone()).ok()?,
            airline: serde_json::from_value(json["airline"].clone()).ok()?,
            time: serde_json::from_value(json["time"].clone()).ok()?,
            duration: serde_json::from_value(json["duration"].clone()).ok()?,
            flight_type: serde_json::from_value(json["flightType"].clone()).ok()?,
            price_inr: serde_json::from_value(json["price_inr"].clone()).ok()?,
            origin_country: serde_json::from_value(json["originCountry"].clone()).ok()?,
            destination_country: serde_json::from_value(json["destinationCountry"].clone()).ok()?,
            rain_probability: serde_json::from_value(json["rainProbability"].clone()).ok()?,
        })
    }
}

pub type FlightDb = Arc<Vec<Flight>>;

pub async fn load_flights() -> Result<FlightDb, std::io::Error> {
    let json_file_path = "flight-price.json";
    if !Path::new(json_file_path).exists() {
        println!("Warning: {} does not exist. Using sample data.", json_file_path);
        
        // Return a sample flight for testing
        let sample_flight = Flight {
            uuid: Uuid::parse_str("39173ac0-bbf4-4b2f-8b89-7f15c3614937").unwrap(),
            date: NaiveDate::parse_from_str("2026-01-04", "%Y-%m-%d").unwrap(),
            origin: "Ho Chi Minh City".to_string(),
            destination: "Ahmedabad".to_string(),
            airline: "Singapore Airlines".to_string(),
            time: "19:50 - 21:50".to_string(),
            duration: "27h 30m".to_string(),
            flight_type: "1 Stop".to_string(),
            price_inr: 31260,
            origin_country: "Vietnam".to_string(),
            destination_country: "India".to_string(),
            rain_probability: 0,
        };
        
        return Ok(Arc::new(vec![sample_flight]));
    }

    // Read the JSON file
    let file_content = tokio::fs::read_to_string(json_file_path).await?;
    let json_data: serde_json::Value = serde_json::from_str(&file_content)?;
    
    // Parse the JSON array
    if let Some(json_array) = json_data.as_array() {
        let mut flights = Vec::new();
        
        for flight_json in json_array {
            if let Some(flight) = Flight::from_json(flight_json) {
                flights.push(flight);
            }
        }
        
        Ok(Arc::new(flights))
    } else {
        // If not an array, return empty vector
        Ok(Arc::new(Vec::new()))
    }
}
