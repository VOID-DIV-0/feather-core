//! # Text Module for Nekonomicon
//!
//! String and pattern operations for text manipulation.
//! Provides functions to create, transform, inspect, match, and format textual values.
//!
//! All operations work with String records.

use std::error::Error;
use std::fmt;
use regex::Regex;

/// Error types for text operations
#[derive(Debug)]
pub enum TextError {
    /// Invalid regex pattern
    InvalidPattern(String),
    /// Index out of bounds
    IndexOutOfBounds,
    /// Invalid format string
    InvalidFormat(String),
}

impl fmt::Display for TextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextError::InvalidPattern(p) => write!(f, "TEXT-001: Invalid regex pattern: {}", p),
            TextError::IndexOutOfBounds => write!(f, "TEXT-002: Index out of bounds"),
            TextError::InvalidFormat(s) => write!(f, "TEXT-003: Invalid format: {}", s),
        }
    }
}

impl Error for TextError {}

/// Concatenate multiple strings
pub fn concat(parts: &[&str]) -> String {
    parts.join("")
}

/// Get length of string
pub fn length(s: &str) -> usize {
    s.chars().count()
}

/// Convert string to uppercase
pub fn upper(s: &str) -> String {
    s.to_uppercase()
}

/// Convert string to lowercase
pub fn lower(s: &str) -> String {
    s.to_lowercase()
}

/// Split string by delimiter
pub fn split(s: &str, delimiter: &str) -> Vec<String> {
    s.split(delimiter)
        .map(|s| s.to_string())
        .collect()
}

/// Join strings with separator
pub fn join(parts: &[String], separator: &str) -> String {
    parts.join(separator)
}

/// Check if string matches regex pattern
pub fn matches(s: &str, pattern: &str) -> Result<bool, TextError> {
    let re = Regex::new(pattern)
        .map_err(|e| TextError::InvalidPattern(format!("{}: {}", pattern, e)))?;
    Ok(re.is_match(s))
}

/// Replace all occurrences of pattern with replacement
pub fn replace(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

/// Replace using regex pattern
pub fn replace_regex(s: &str, pattern: &str, replacement: &str) -> Result<String, TextError> {
    let re = Regex::new(pattern)
        .map_err(|e| TextError::InvalidPattern(format!("{}: {}", pattern, e)))?;
    Ok(re.replace_all(s, replacement).to_string())
}

/// Capture groups from regex pattern
pub fn capture(s: &str, pattern: &str) -> Result<Vec<String>, TextError> {
    let re = Regex::new(pattern)
        .map_err(|e| TextError::InvalidPattern(format!("{}: {}", pattern, e)))?;
    
    let mut captures = Vec::new();
    if let Some(caps) = re.captures(s) {
        for i in 0..caps.len() {
            if let Some(m) = caps.get(i) {
                captures.push(m.as_str().to_string());
            }
        }
    }
    
    Ok(captures)
}

/// Get substring from start to end indices (inclusive)
pub fn slice(s: &str, start: usize, end: usize) -> Result<String, TextError> {
    let chars: Vec<char> = s.chars().collect();
    
    if start >= chars.len() || end >= chars.len() || start > end {
        return Err(TextError::IndexOutOfBounds);
    }
    
    Ok(chars[start..=end].iter().collect())
}

/// Trim whitespace from both ends
pub fn trim(s: &str) -> String {
    s.trim().to_string()
}

/// Trim whitespace from start
pub fn trim_start(s: &str) -> String {
    s.trim_start().to_string()
}

/// Trim whitespace from end
pub fn trim_end(s: &str) -> String {
    s.trim_end().to_string()
}

/// Check if string starts with prefix
pub fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

/// Check if string ends with suffix
pub fn ends_with(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

/// Check if string contains substring
pub fn contains(s: &str, substring: &str) -> bool {
    s.contains(substring)
}

/// Repeat string n times
pub fn repeat(s: &str, count: usize) -> String {
    s.repeat(count)
}

/// Reverse string
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// Pad string to length with character (left padding)
pub fn pad_left(s: &str, width: usize, pad_char: char) -> String {
    let char_count = s.chars().count();
    if char_count >= width {
        s.to_string()
    } else {
        let padding = pad_char.to_string().repeat(width - char_count);
        format!("{}{}", padding, s)
    }
}

/// Pad string to length with character (right padding)
pub fn pad_right(s: &str, width: usize, pad_char: char) -> String {
    let char_count = s.chars().count();
    if char_count >= width {
        s.to_string()
    } else {
        let padding = pad_char.to_string().repeat(width - char_count);
        format!("{}{}", s, padding)
    }
}

/// Compare two strings (case-sensitive)
pub fn compare(a: &str, b: &str) -> bool {
    a == b
}

/// Compare two strings (case-insensitive)
pub fn compare_ignore_case(a: &str, b: &str) -> bool {
    a.to_lowercase() == b.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(concat(&["hello", " ", "world"]), "hello world");
        assert_eq!(concat(&["a", "b", "c"]), "abc");
    }

    #[test]
    fn test_length() {
        assert_eq!(length("hello"), 5);
        assert_eq!(length(""), 0);
        assert_eq!(length("こんにちは"), 5); // Unicode characters
    }

    #[test]
    fn test_upper_lower() {
        assert_eq!(upper("hello"), "HELLO");
        assert_eq!(lower("HELLO"), "hello");
        assert_eq!(upper("Hello World"), "HELLO WORLD");
    }

    #[test]
    fn test_split() {
        let result = split("a,b,c", ",");
        assert_eq!(result, vec!["a", "b", "c"]);
        
        let result = split("hello world", " ");
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn test_join() {
        let parts = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(join(&parts, ", "), "a, b, c");
        assert_eq!(join(&parts, ""), "abc");
    }

    #[test]
    fn test_matches() {
        assert!(matches("hello123", r"hello\d+").unwrap());
        assert!(!matches("hello", r"hello\d+").unwrap());
        assert!(matches("test@example.com", r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap());
    }

    #[test]
    fn test_replace() {
        assert_eq!(replace("hello world", "world", "rust"), "hello rust");
        assert_eq!(replace("aaa", "a", "b"), "bbb");
    }

    #[test]
    fn test_replace_regex() {
        let result = replace_regex("hello123world456", r"\d+", "X").unwrap();
        assert_eq!(result, "helloXworldX");
    }

    #[test]
    fn test_capture() {
        let result = capture("ABC-123", r"([A-Z]+)-(\d+)").unwrap();
        assert_eq!(result.len(), 3); // Full match + 2 groups
        assert_eq!(result[0], "ABC-123");
        assert_eq!(result[1], "ABC");
        assert_eq!(result[2], "123");
    }

    #[test]
    fn test_slice() {
        assert_eq!(slice("hello", 0, 4).unwrap(), "hello");
        assert_eq!(slice("hello", 1, 3).unwrap(), "ell");
        assert_eq!(slice("hello", 0, 0).unwrap(), "h");
        assert!(slice("hello", 10, 20).is_err());
    }

    #[test]
    fn test_trim() {
        assert_eq!(trim("  hello  "), "hello");
        assert_eq!(trim_start("  hello  "), "hello  ");
        assert_eq!(trim_end("  hello  "), "  hello");
    }

    #[test]
    fn test_starts_ends_contains() {
        assert!(starts_with("hello world", "hello"));
        assert!(!starts_with("hello world", "world"));
        
        assert!(ends_with("hello world", "world"));
        assert!(!ends_with("hello world", "hello"));
        
        assert!(contains("hello world", "lo wo"));
        assert!(!contains("hello world", "xyz"));
    }

    #[test]
    fn test_repeat() {
        assert_eq!(repeat("ab", 3), "ababab");
        assert_eq!(repeat("x", 5), "xxxxx");
        assert_eq!(repeat("test", 0), "");
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse("abc"), "cba");
        assert_eq!(reverse(""), "");
    }

    #[test]
    fn test_padding() {
        assert_eq!(pad_left("5", 3, '0'), "005");
        assert_eq!(pad_right("5", 3, '0'), "500");
        assert_eq!(pad_left("hello", 3, '0'), "hello"); // No padding needed
        
        // Unicode character test
        assert_eq!(pad_left("こ", 3, '0'), "00こ"); // One Unicode char, pad to 3
        assert_eq!(pad_right("こ", 3, '0'), "こ00"); // One Unicode char, pad to 3
    }

    #[test]
    fn test_compare() {
        assert!(compare("hello", "hello"));
        assert!(!compare("hello", "Hello"));
        
        assert!(compare_ignore_case("hello", "HELLO"));
        assert!(compare_ignore_case("Hello", "hello"));
    }
}
