//! # Math Module for Nekonomicon
//!
//! Mathematical operations and utilities for numeric calculations.
//! Provides arithmetic, comparison, rounding, and random number operations.
//!
//! All operations work with records (strings) that are lensed to numbers internally.

use std::error::Error;
use std::fmt;
use rand::Rng;

/// Error types for math operations
#[derive(Debug)]
pub enum MathError {
    /// Division by zero error
    DivisionByZero,
    /// Invalid numeric format
    InvalidFormat(String),
    /// Overflow or underflow error
    Overflow,
    /// Invalid comparison operator
    InvalidOperator(String),
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "MATH-001: Division by zero"),
            MathError::InvalidFormat(s) => write!(f, "MATH-002: Invalid numeric format: {}", s),
            MathError::Overflow => write!(f, "MATH-003: Overflow or underflow"),
            MathError::InvalidOperator(s) => write!(f, "MATH-004: Invalid operator: {}", s),
        }
    }
}

impl Error for MathError {}

/// Parse a record (string) to f64
fn parse_number(s: &str) -> Result<f64, MathError> {
    s.trim()
        .parse::<f64>()
        .map_err(|_| MathError::InvalidFormat(s.to_string()))
}

/// Add two numbers
pub fn add(a: &str, b: &str) -> Result<String, MathError> {
    let num_a = parse_number(a)?;
    let num_b = parse_number(b)?;
    let result = num_a + num_b;
    
    if result.is_infinite() {
        return Err(MathError::Overflow);
    }
    
    Ok(format_number(result))
}

/// Subtract two numbers
pub fn sub(a: &str, b: &str) -> Result<String, MathError> {
    let num_a = parse_number(a)?;
    let num_b = parse_number(b)?;
    let result = num_a - num_b;
    
    if result.is_infinite() {
        return Err(MathError::Overflow);
    }
    
    Ok(format_number(result))
}

/// Multiply two numbers
pub fn mul(a: &str, b: &str) -> Result<String, MathError> {
    let num_a = parse_number(a)?;
    let num_b = parse_number(b)?;
    let result = num_a * num_b;
    
    if result.is_infinite() {
        return Err(MathError::Overflow);
    }
    
    Ok(format_number(result))
}

/// Divide two numbers
pub fn div(a: &str, b: &str) -> Result<String, MathError> {
    let num_a = parse_number(a)?;
    let num_b = parse_number(b)?;
    
    if num_b == 0.0 {
        return Err(MathError::DivisionByZero);
    }
    
    let result = num_a / num_b;
    
    if result.is_infinite() {
        return Err(MathError::Overflow);
    }
    
    Ok(format_number(result))
}

/// Calculate modulo (remainder)
pub fn modulo(a: &str, b: &str) -> Result<String, MathError> {
    let num_a = parse_number(a)?;
    let num_b = parse_number(b)?;
    
    if num_b == 0.0 {
        return Err(MathError::DivisionByZero);
    }
    
    let result = num_a % num_b;
    Ok(format_number(result))
}

/// Get absolute value
pub fn abs(n: &str) -> Result<String, MathError> {
    let num = parse_number(n)?;
    Ok(format_number(num.abs()))
}

/// Round to nearest integer
pub fn round(n: &str) -> Result<String, MathError> {
    let num = parse_number(n)?;
    Ok(format_number(num.round()))
}

/// Round down to integer (floor)
pub fn floor(n: &str) -> Result<String, MathError> {
    let num = parse_number(n)?;
    Ok(format_number(num.floor()))
}

/// Round up to integer (ceil)
pub fn ceil(n: &str) -> Result<String, MathError> {
    let num = parse_number(n)?;
    Ok(format_number(num.ceil()))
}

/// Get minimum of two values
pub fn min(a: &str, b: &str) -> Result<String, MathError> {
    let num_a = parse_number(a)?;
    let num_b = parse_number(b)?;
    Ok(format_number(num_a.min(num_b)))
}

/// Get maximum of two values
pub fn max(a: &str, b: &str) -> Result<String, MathError> {
    let num_a = parse_number(a)?;
    let num_b = parse_number(b)?;
    Ok(format_number(num_a.max(num_b)))
}

/// Constrain value to range
pub fn clamp(n: &str, low: &str, high: &str) -> Result<String, MathError> {
    let num = parse_number(n)?;
    let min_val = parse_number(low)?;
    let max_val = parse_number(high)?;
    
    let result = num.max(min_val).min(max_val);
    Ok(format_number(result))
}

/// Raise to power
pub fn pow(n: &str, exp: &str) -> Result<String, MathError> {
    let num = parse_number(n)?;
    let exponent = parse_number(exp)?;
    let result = num.powf(exponent);
    
    if result.is_infinite() || result.is_nan() {
        return Err(MathError::Overflow);
    }
    
    Ok(format_number(result))
}

/// Compare two numbers
/// Operators: >, <, >=, <=, ==, !=
pub fn compare(a: &str, operator: &str, b: &str) -> Result<String, MathError> {
    let num_a = parse_number(a)?;
    let num_b = parse_number(b)?;
    
    let result = match operator {
        ">" => num_a > num_b,
        "<" => num_a < num_b,
        ">=" => num_a >= num_b,
        "<=" => num_a <= num_b,
        "==" => (num_a - num_b).abs() < f64::EPSILON,
        "!=" => (num_a - num_b).abs() >= f64::EPSILON,
        _ => return Err(MathError::InvalidOperator(operator.to_string())),
    };
    
    Ok(if result { "true".to_string() } else { "false".to_string() })
}

/// Generate random number in range [min, max]
pub fn random(min: &str, max: &str) -> Result<String, MathError> {
    let min_val = parse_number(min)?;
    let max_val = parse_number(max)?;
    
    let mut rng = rand::thread_rng();
    let result = rng.gen_range(min_val..=max_val);
    
    Ok(format_number(result))
}

/// Format number for output, removing unnecessary decimal places
fn format_number(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{:.0}", n)
    } else {
        // Remove trailing zeros after decimal point
        let formatted = format!("{}", n);
        formatted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add("10", "5").unwrap(), "15");
        assert_eq!(add("10.5", "5.5").unwrap(), "16");
        assert_eq!(add("-10", "5").unwrap(), "-5");
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub("10", "5").unwrap(), "5");
        assert_eq!(sub("5", "10").unwrap(), "-5");
        assert_eq!(sub("10.5", "5.5").unwrap(), "5");
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul("10", "5").unwrap(), "50");
        assert_eq!(mul("10", "-5").unwrap(), "-50");
        assert_eq!(mul("2.5", "4").unwrap(), "10");
    }

    #[test]
    fn test_div() {
        assert_eq!(div("10", "5").unwrap(), "2");
        assert_eq!(div("10", "2").unwrap(), "5");
        assert!(div("10", "0").is_err());
    }

    #[test]
    fn test_modulo() {
        assert_eq!(modulo("10", "3").unwrap(), "1");
        assert_eq!(modulo("10", "5").unwrap(), "0");
    }

    #[test]
    fn test_abs() {
        assert_eq!(abs("-42").unwrap(), "42");
        assert_eq!(abs("42").unwrap(), "42");
    }

    #[test]
    fn test_rounding() {
        assert_eq!(round("3.7").unwrap(), "4");
        assert_eq!(round("3.4").unwrap(), "3");
        assert_eq!(floor("3.7").unwrap(), "3");
        assert_eq!(ceil("3.4").unwrap(), "4");
    }

    #[test]
    fn test_min_max() {
        assert_eq!(min("10", "5").unwrap(), "5");
        assert_eq!(max("10", "5").unwrap(), "10");
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp("15", "0", "10").unwrap(), "10");
        assert_eq!(clamp("5", "0", "10").unwrap(), "5");
        assert_eq!(clamp("-5", "0", "10").unwrap(), "0");
    }

    #[test]
    fn test_pow() {
        assert_eq!(pow("2", "3").unwrap(), "8");
        assert_eq!(pow("10", "2").unwrap(), "100");
    }

    #[test]
    fn test_compare() {
        assert_eq!(compare("10", ">", "5").unwrap(), "true");
        assert_eq!(compare("5", ">", "10").unwrap(), "false");
        assert_eq!(compare("10", "==", "10").unwrap(), "true");
        assert_eq!(compare("10", "!=", "5").unwrap(), "true");
    }

    #[test]
    fn test_invalid_format() {
        assert!(add("abc", "5").is_err());
        assert!(add("10", "xyz").is_err());
    }

    #[test]
    fn test_random() {
        let result = random("1", "10").unwrap();
        let num = result.parse::<f64>().unwrap();
        assert!(num >= 1.0 && num <= 10.0);
    }
}
