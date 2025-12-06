# nekonomicon-core-http

HTTP requests and web communication for the nekonomicon scripting language.

## Overview

The HTTP module provides functionality for making HTTP requests, handling responses, and managing web communication. It supports all common HTTP methods and includes features for headers, request bodies, and JSON handling.

## Features

### HTTP Methods
- **get**: Perform GET request
- **post**: Perform POST request with optional body
- **put**: Perform PUT request with optional body
- **delete**: Perform DELETE request
- **patch**: Perform PATCH request with optional body

### Specialized Methods
- **post_json**: POST with JSON content type
- **put_json**: PUT with JSON content type

### Response Handling
- **HttpResponse**: Structure containing status, body, and headers
- Status code checking with `is_success()`
- Header access with `get_header()`

## Usage Example

```rust
use nekonomicon_core_http::*;
use std::collections::HashMap;

// Simple GET request
let response = get("https://api.example.com/users", None).unwrap();
println!("Status: {}", response.status);
println!("Body: {}", response.body);

// GET with headers
let mut headers = HashMap::new();
headers.insert("Authorization".to_string(), "Bearer token123".to_string());
let response = get("https://api.example.com/protected", Some(headers)).unwrap();

// POST with JSON
let json_body = r#"{"name": "John", "email": "john@example.com"}"#;
let response = post_json("https://api.example.com/users", json_body, None).unwrap();

if response.is_success() {
    println!("Success! Status: {}", response.status);
} else {
    println!("Failed with status: {}", response.status);
}

// POST with custom body and headers
let mut headers = HashMap::new();
headers.insert("Content-Type".to_string(), "text/plain".to_string());
let body = "Raw text content";
let response = post(
    "https://api.example.com/data",
    Some(body.to_string()),
    Some(headers)
).unwrap();

// Access response headers
if let Some(content_type) = response.get_header("content-type") {
    println!("Content-Type: {}", content_type);
}

// PUT request
let json_body = r#"{"status": "updated"}"#;
let response = put_json(
    "https://api.example.com/users/123",
    json_body,
    None
).unwrap();

// DELETE request
let response = delete("https://api.example.com/users/123", None).unwrap();
```

## HttpResponse Structure

The `HttpResponse` struct provides:
- `status`: HTTP status code (u16)
- `body`: Response body as string
- `headers`: HashMap of response headers
- `is_success()`: Returns true for 2xx status codes
- `get_header(name)`: Get header value by name

```rust
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl HttpResponse {
    pub fn is_success(&self) -> bool { /* ... */ }
    pub fn get_header(&self, name: &str) -> Option<&String> { /* ... */ }
}
```

## Error Handling

The module defines several error types:
- **HTTP-001**: Request failed
- **HTTP-002**: Invalid URL
- **HTTP-003**: Invalid header
- **HTTP-004**: Response parsing error
- **HTTP-005**: Request timeout
- **HTTP-006**: Network error

All operations return `Result<HttpResponse, HttpError>`.

## Common Patterns

### REST API Calls
```rust
// GET list of resources
let response = get("https://api.example.com/items", None).unwrap();

// POST to create new resource
let json = r#"{"name": "New Item"}"#;
let response = post_json("https://api.example.com/items", json, None).unwrap();

// PUT to update resource
let json = r#"{"name": "Updated Item"}"#;
let response = put_json("https://api.example.com/items/1", json, None).unwrap();

// DELETE to remove resource
let response = delete("https://api.example.com/items/1", None).unwrap();
```

### Authentication Headers
```rust
let mut headers = HashMap::new();
headers.insert("Authorization".to_string(), "Bearer YOUR_TOKEN".to_string());
headers.insert("X-API-Key".to_string(), "your-api-key".to_string());

let response = get("https://api.example.com/protected", Some(headers)).unwrap();
```

### Custom Content Types
```rust
let mut headers = HashMap::new();
headers.insert("Content-Type".to_string(), "application/xml".to_string());

let xml_body = "<user><name>John</name></user>";
let response = post(
    "https://api.example.com/users",
    Some(xml_body.to_string()),
    Some(headers)
).unwrap();
```

## Testing

Run tests with:
```bash
cargo test
```

The module includes unit tests for:
- Response creation and inspection
- Header parsing
- Status code checking
- Error handling

Note: Integration tests with actual HTTP requests are not included in unit tests but can be added separately.

## nekonomicon Script Usage

In nekonomicon scripts, the HTTP module is used as:

```spell
http get 'https://api.example.com/users' into ::response.
say ::response:status.
say ::response:body.

http post 'https://api.example.com/users'
  with body '{"name": "John"}'
  with headers '{"Content-Type": "application/json"}'
  into ::result.
```

See the [HTTP module documentation](../../docs/modules/network/http.md) for complete nekonomicon language syntax and examples.

## Best Practices

- Always check `is_success()` before processing response body
- Use appropriate content-type headers for different body formats
- Handle errors gracefully (network issues, timeouts, etc.)
- Set reasonable timeout values for long-running requests
- Reuse client instances when making multiple requests (in custom code)
- Validate URLs before making requests
- Use HTTPS for sensitive data

## Security Considerations

- Never log or expose sensitive headers (Authorization, API keys)
- Validate SSL certificates (enabled by default)
- Use `sensitive` clause in nekonomicon when handling authentication data
- Be cautious with user-provided URLs to prevent SSRF attacks
- Follow the principle of least privilege when granting network access

## Dependencies

- `reqwest`: HTTP client library with blocking interface
- `serde` and `serde_json`: JSON serialization/deserialization

## License

Part of the nekonomicon project. See LICENSE file for details.
