//! # HTTP Module for Nekonomicon
//!
//! HTTP requests and web communication.
//! Provides functionality for making HTTP requests, handling responses,
//! and managing web communication in nekonomicon scripts.
//!
//! Supports GET, POST, PUT, DELETE, and PATCH methods with headers and body.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

/// Error types for HTTP operations
#[derive(Debug)]
pub enum HttpError {
    /// Request failed
    RequestFailed(String),
    /// Invalid URL
    InvalidUrl(String),
    /// Invalid header
    InvalidHeader(String),
    /// Response parsing error
    ParseError(String),
    /// Timeout error
    Timeout,
    /// Network error
    NetworkError(String),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::RequestFailed(msg) => write!(f, "HTTP-001: Request failed: {}", msg),
            HttpError::InvalidUrl(url) => write!(f, "HTTP-002: Invalid URL: {}", url),
            HttpError::InvalidHeader(h) => write!(f, "HTTP-003: Invalid header: {}", h),
            HttpError::ParseError(msg) => write!(f, "HTTP-004: Parse error: {}", msg),
            HttpError::Timeout => write!(f, "HTTP-005: Request timeout"),
            HttpError::NetworkError(msg) => write!(f, "HTTP-006: Network error: {}", msg),
        }
    }
}

impl Error for HttpError {}

impl From<reqwest::Error> for HttpError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            HttpError::Timeout
        } else if err.is_connect() {
            HttpError::NetworkError(err.to_string())
        } else {
            HttpError::RequestFailed(err.to_string())
        }
    }
}

/// HTTP response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    /// HTTP status code
    pub status: u16,
    /// Response body as string
    pub body: String,
    /// Response headers
    pub headers: HashMap<String, String>,
}

impl HttpResponse {
    /// Create a new HTTP response
    pub fn new(status: u16, body: String, headers: HashMap<String, String>) -> Self {
        HttpResponse {
            status,
            body,
            headers,
        }
    }

    /// Check if response is successful (2xx status code)
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// Get header value by name
    pub fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    }
}

/// Convert reqwest Response to HttpResponse
fn parse_response(response: Response) -> Result<HttpResponse, HttpError> {
    let status = response.status().as_u16();
    
    let mut headers = HashMap::new();
    for (name, value) in response.headers().iter() {
        if let Ok(value_str) = value.to_str() {
            headers.insert(name.to_string(), value_str.to_string());
        }
    }
    
    let body = response
        .text()
        .map_err(|e| HttpError::ParseError(e.to_string()))?;
    
    Ok(HttpResponse::new(status, body, headers))
}

/// Parse headers from HashMap
fn parse_headers(headers: &HashMap<String, String>) -> Result<HeaderMap, HttpError> {
    let mut header_map = HeaderMap::new();
    
    for (key, value) in headers {
        let header_name = HeaderName::from_bytes(key.as_bytes())
            .map_err(|e| HttpError::InvalidHeader(format!("{}: {}", key, e)))?;
        let header_value = HeaderValue::from_str(value)
            .map_err(|e| HttpError::InvalidHeader(format!("{}: {}", value, e)))?;
        header_map.insert(header_name, header_value);
    }
    
    Ok(header_map)
}

/// Perform HTTP GET request
pub fn get(url: &str, headers: Option<HashMap<String, String>>) -> Result<HttpResponse, HttpError> {
    let client = Client::new();
    let mut request = client.get(url);
    
    if let Some(h) = headers {
        request = request.headers(parse_headers(&h)?);
    }
    
    let response = request.send()?;
    parse_response(response)
}

/// Perform HTTP POST request
pub fn post(
    url: &str,
    body: Option<String>,
    headers: Option<HashMap<String, String>>,
) -> Result<HttpResponse, HttpError> {
    let client = Client::new();
    let mut request = client.post(url);
    
    if let Some(h) = headers {
        request = request.headers(parse_headers(&h)?);
    }
    
    if let Some(b) = body {
        request = request.body(b);
    }
    
    let response = request.send()?;
    parse_response(response)
}

/// Perform HTTP PUT request
pub fn put(
    url: &str,
    body: Option<String>,
    headers: Option<HashMap<String, String>>,
) -> Result<HttpResponse, HttpError> {
    let client = Client::new();
    let mut request = client.put(url);
    
    if let Some(h) = headers {
        request = request.headers(parse_headers(&h)?);
    }
    
    if let Some(b) = body {
        request = request.body(b);
    }
    
    let response = request.send()?;
    parse_response(response)
}

/// Perform HTTP DELETE request
pub fn delete(
    url: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<HttpResponse, HttpError> {
    let client = Client::new();
    let mut request = client.delete(url);
    
    if let Some(h) = headers {
        request = request.headers(parse_headers(&h)?);
    }
    
    let response = request.send()?;
    parse_response(response)
}

/// Perform HTTP PATCH request
pub fn patch(
    url: &str,
    body: Option<String>,
    headers: Option<HashMap<String, String>>,
) -> Result<HttpResponse, HttpError> {
    let client = Client::new();
    let mut request = client.patch(url);
    
    if let Some(h) = headers {
        request = request.headers(parse_headers(&h)?);
    }
    
    if let Some(b) = body {
        request = request.body(b);
    }
    
    let response = request.send()?;
    parse_response(response)
}

/// POST with JSON body
pub fn post_json(
    url: &str,
    json: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<HttpResponse, HttpError> {
    let mut all_headers = headers.unwrap_or_default();
    all_headers.insert("Content-Type".to_string(), "application/json".to_string());
    
    post(url, Some(json.to_string()), Some(all_headers))
}

/// PUT with JSON body
pub fn put_json(
    url: &str,
    json: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<HttpResponse, HttpError> {
    let mut all_headers = headers.unwrap_or_default();
    all_headers.insert("Content-Type".to_string(), "application/json".to_string());
    
    put(url, Some(json.to_string()), Some(all_headers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_response_creation() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        let response = HttpResponse::new(200, "test body".to_string(), headers);
        
        assert_eq!(response.status, 200);
        assert_eq!(response.body, "test body");
        assert!(response.is_success());
        assert_eq!(
            response.get_header("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_is_success() {
        let response_200 = HttpResponse::new(200, "".to_string(), HashMap::new());
        assert!(response_200.is_success());
        
        let response_201 = HttpResponse::new(201, "".to_string(), HashMap::new());
        assert!(response_201.is_success());
        
        let response_404 = HttpResponse::new(404, "".to_string(), HashMap::new());
        assert!(!response_404.is_success());
        
        let response_500 = HttpResponse::new(500, "".to_string(), HashMap::new());
        assert!(!response_500.is_success());
    }

    #[test]
    fn test_get_header() {
        let mut headers = HashMap::new();
        headers.insert("X-Custom-Header".to_string(), "custom-value".to_string());
        
        let response = HttpResponse::new(200, "".to_string(), headers);
        
        assert_eq!(
            response.get_header("X-Custom-Header"),
            Some(&"custom-value".to_string())
        );
        assert_eq!(response.get_header("Non-Existent"), None);
    }

    #[test]
    fn test_parse_headers() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        
        let header_map = parse_headers(&headers).unwrap();
        
        assert_eq!(header_map.len(), 2);
        assert!(header_map.contains_key("content-type"));
        assert!(header_map.contains_key("authorization"));
    }

    #[test]
    fn test_invalid_header() {
        let mut headers = HashMap::new();
        headers.insert("Invalid\nHeader".to_string(), "value".to_string());
        
        let result = parse_headers(&headers);
        assert!(result.is_err());
    }

    // Note: Integration tests requiring actual HTTP requests would be in a separate test file
    // and might use a mock HTTP server for testing
}
