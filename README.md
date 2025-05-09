# Flight API Documentation

## Endpoints

### Get Flights
`GET /api/flights`

Returns paginated flight data with optional filtering and sorting.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| page | integer | No | Page number (default: 1) |
| origin | string | No | Filter by origin city/airport |
| destination | string | No | Filter by destination city/airport |
| sort_by | string | No | Sort method: "price" (default) or "date" |
| max_price | integer | No | Filter flights with price under this value |
| max_rain | integer | No | Filter flights with rain probability under this value |

#### Response Format

```json
{
  "data": [
    {
      "uuid": "string",
      "date": "YYYY-MM-DD",
      "origin": "string",
      "destination": "string",
      "airline": "string",
      "time": "string",
      "duration": "string",
      "flight_type": "string",
      "price_inr": number,
      "origin_country": "string",
      "destination_country": "string",
      "rain_probability": number
    }
  ],
  "page": number,
  "total_pages": number,
  "total_items": number
}
```

#### Examples

- Get first page of flights sorted by price (default):
  `GET /api/flights`

- Get flights from New Delhi to Hanoi with price under 10000:
  `GET /api/flights?origin=New%20Delhi&destination=Hanoi&max_price=10000`

- Get flights sorted by date with rain probability under 20:
  `GET /api/flights?sort_by=date&max_rain=20`

- Get second page of results:
  `GET /api/flights?page=2`

Notes:
- Pagination returns up to 20 items per page
- Origin/destination matching is case-insensitive and partial (contains search)
- Multiple filters can be applied simultaneously
- Sort order for price is ascending (lowest first)
- Sort order for date is chronological (earliest first)
