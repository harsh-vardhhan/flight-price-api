use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use crate::models::{Flight, FlightDb};

#[derive(Debug, Deserialize)]
pub struct FlightParams {
    page: Option<usize>,
    origin: Option<String>,
    destination: Option<String>,
    airline: Option<String>,
    sort_by: Option<String>,
    max_price: Option<u32>, 
    max_rain: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse {
    data: Vec<Flight>,
    page: usize,
    total_pages: usize,
    total_items: usize,
}

pub async fn list_flights(
    State(flights): State<FlightDb>,
    Query(params): Query<FlightParams>,
) -> Json<PaginatedResponse> {
    // Default page size
    const PAGE_SIZE: usize = 20;
    
    // Get page number, default to 1
    let page = params.page.unwrap_or(1);
    
    // Apply filters
    let mut filtered_flights = flights.to_vec();
    
    // Filter by origin if specified
    if let Some(origin) = params.origin {
        filtered_flights.retain(|flight| flight.origin.to_lowercase().contains(&origin.to_lowercase()));
    }
    
    // Filter by destination if specified
    if let Some(destination) = params.destination {
        filtered_flights.retain(|flight| flight.destination.to_lowercase().contains(&destination.to_lowercase()));
    }
    
    // Filter by airline if specified
    if let Some(airline) = params.airline {
        // Split by comma to handle multiple airlines
        let airlines: Vec<&str> = airline.split(',').collect();
        filtered_flights.retain(|flight| {
            airlines.iter().any(|&a| flight.airline.to_lowercase().contains(&a.trim().to_lowercase()))
        });
    }
    
    // Filter by max price if specified
    if let Some(max_price) = params.max_price {
        filtered_flights.retain(|flight| flight.price_inr < max_price);
    }
    
    // Filter by max rain probability if specified
    if let Some(max_rain) = params.max_rain {
        filtered_flights.retain(|flight| flight.rain_probability < max_rain);
    }
    
    // Sort based on sort parameter
    match params.sort_by.as_deref() {
        Some("date") => {
            // Sort by date (ascending)
            filtered_flights.sort_by(|a, b| a.date.cmp(&b.date));
        },
        _ => {
            // Default: Sort by price (ascending)
            filtered_flights.sort_by(|a, b| a.price_inr.cmp(&b.price_inr));
        }
    }
    
    // Calculate pagination
    let total_items = filtered_flights.len();
    let total_pages = (total_items + PAGE_SIZE - 1) / PAGE_SIZE; // Ceiling division
    
    // Get the requested page
    let start = (page - 1) * PAGE_SIZE;
    let end = (start + PAGE_SIZE).min(total_items);
    
    let paginated_data = if start < total_items {
        filtered_flights[start..end].to_vec()
    } else {
        Vec::new()
    };
    
    Json(PaginatedResponse {
        data: paginated_data,
        page,
        total_pages,
        total_items,
    })
}