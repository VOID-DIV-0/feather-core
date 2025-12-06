# nekonomicon-core-math

Mathematical operations and utilities for the nekonomicon scripting language.

## Overview

The Math module provides comprehensive numerical computation capabilities, including:
- Basic arithmetic (add, subtract, multiply, divide, modulo)
- Rounding operations (round, floor, ceil)
- Value comparison and selection (min, max, clamp)
- Advanced operations (power, absolute value)
- Comparison operators (>, <, >=, <=, ==, !=)
- Random number generation

All operations work with string records that are automatically parsed to numbers internally.

## Features

### Arithmetic Operations
- **add**: Add two numbers
- **sub**: Subtract two numbers
- **mul**: Multiply two numbers
- **div**: Divide two numbers (with division-by-zero protection)
- **modulo**: Calculate remainder

### Rounding Operations
- **abs**: Get absolute value
- **round**: Round to nearest integer
- **floor**: Round down to integer
- **ceil**: Round up to integer

### Value Selection
- **min**: Get minimum of two values
- **max**: Get maximum of two values
- **clamp**: Constrain value to a range

### Advanced Operations
- **pow**: Raise number to power
- **compare**: Compare two numbers with various operators
- **random**: Generate random number in range

## Usage Example

```rust
use nekonomicon_core_math::*;

// Basic arithmetic
let result = add("10", "5").unwrap(); // "15"
let result = mul("3", "7").unwrap();  // "21"

// Rounding
let result = round("3.7").unwrap();   // "4"
let result = floor("3.7").unwrap();   // "3"

// Comparisons
let result = compare("10", ">", "5").unwrap(); // "true"
let result = max("10", "5").unwrap();          // "10"

// Random numbers
let result = random("1", "10").unwrap(); // Random value between 1 and 10
```

## Error Handling

The module defines several error types:
- **MATH-001**: Division by zero
- **MATH-002**: Invalid numeric format
- **MATH-003**: Overflow or underflow
- **MATH-004**: Invalid comparison operator

All operations return `Result<String, MathError>` for proper error handling.

## Testing

Run tests with:
```bash
cargo test
```

All operations include comprehensive unit tests covering normal cases, edge cases, and error conditions.

## nekonomicon Script Usage

In nekonomicon scripts, the math module is used as:

```spell
math add '10' '5' into @sum.
math compare '10' '>' '5' into @is_greater.
math random min '1' max '10' into @random_num.
```

See the [Math module documentation](../../docs/modules/utility/math.md) for complete nekonomicon language syntax and examples.

## License

Part of the nekonomicon project. See LICENSE file for details.
