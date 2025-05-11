# Flight API Documentation

## Overview

This API provides access to flight information including routes, airlines, prices, and other details. It offers filtering capabilities and pagination support for efficient data retrieval.

## Base URL

```
http://localhost:3000
```

In production environments, the API will be accessible on all network interfaces (0.0.0.0).

## API Endpoints

### Health Check

Verifies that the API is running properly.

**Endpoint:** `GET /health`

**Response:**
- Status Code: 200 OK
- Content: `"OK"`

### List Flights

Returns a paginated list of flights with optional filtering and sorting capabilities.

**Endpoint:** `GET /api/flights`

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| page | integer | No | Page number (default: 1) |
| origin | string | No | Filter flights by origin city (case-insensitive, partial match) |
| destination | string | No | Filter flights by destination city (case-insensitive, partial match) |
| airline | string | No | Filter flights by airline name(s) (case-insensitive, partial match). Multiple airlines can be specified as comma-separated values. |
| sort_by | string | No | Sort results by field. Options: `"date"` (ascending). Default: sorts by price (ascending) |
| max_price | integer | No | Maximum price in INR |
| max_rain | integer | No | Maximum rain probability (0-100) |

**Response:**

```json
{
  "data": [
    {
      "uuid": "39173ac0-bbf4-4b2f-8b89-7f15c3614937",
      "date": "2026-01-04",
      "origin": "Ho Chi Minh City",
      "destination": "Ahmedabad",
      "airline": "Singapore Airlines",
      "time": "19:50 - 21:50",
      "duration": "27h 30m",
      "flight_type": "1 Stop",
      "price_inr": 31260,
      "origin_country": "Vietnam",
      "destination_country": "India",
      "rain_probability": 0,
      "free_meal": true
    },
    // ... additional flights
  ],
  "page": 1,
  "total_pages": 5,
  "total_items": 87
}
```

#### Response Fields:

| Field | Description |
|-------|-------------|
| data | Array of flight objects |
| page | Current page number |
| total_pages | Total number of pages available |
| total_items | Total number of flights matching the filters |

#### Flight Object Fields:

| Field | Type | Description |
|-------|------|-------------|
| uuid | string | Unique identifier for the flight |
| date | string | Departure date (YYYY-MM-DD format) |
| origin | string | Origin city name |
| destination | string | Destination city name |
| airline | string | Airline name |
| time | string | Departure and arrival time (format: "HH:MM - HH:MM") |
| duration | string | Flight duration (format: "XXh YYm") |
| flight_type | string | Type of flight (e.g., "Non-stop", "1 Stop") |
| price_inr | integer | Price in Indian Rupees (INR) |
| origin_country | string | Country of origin |
| destination_country | string | Destination country |
| rain_probability | integer | Probability of rain at destination (0-100) |
| free_meal | boolean | Whether a free meal is provided on the flight |

## Examples

### Basic Request

```
GET /api/flights
```

Returns the first page of flights, sorted by price.

### Filtered Request

```
GET /api/flights?origin=delhi&destination=mumbai&airline=indigo,air%20india&max_price=5000
```

Returns flights from Delhi to Mumbai, operated by either IndiGo or Air India, with prices less than 5000 INR.

### Pagination

```
GET /api/flights?page=2
```

Returns the second page of flights (items 21-40).

### Sorting

```
GET /api/flights?sort_by=date
```

Returns flights sorted by departure date (ascending).

## Error Handling

The API follows standard HTTP status codes for error responses:

- `200 OK`: Request successful
- `400 Bad Request`: Invalid parameters
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server-side error

## Data Source

Flight data is loaded from a `flight-price.json` file when the API starts. If the file is not present, a sample flight will be used for testing purposes.

## CORS Support

The API supports Cross-Origin Resource Sharing (CORS) with the following configuration:
- All origins are allowed
- All HTTP methods are allowed
- All headers are allowed

This enables the API to be accessed from web browsers across different domains.