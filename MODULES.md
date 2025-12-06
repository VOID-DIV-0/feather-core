# Nekonomicon Core Modules

This document provides an overview of the core modules implemented for the nekonomicon scripting language. Each module is designed to follow nekonomicon's principles of clarity, safety, and composability.

## Overview

Four essential modules have been implemented to provide fundamental functionality:

1. **Math** - Mathematical operations and numeric computations
2. **List** - Ordered collection management (stacks and queues)
3. **Text** - String manipulation and pattern matching
4. **HTTP** - Web communication and REST API interactions

All modules are implemented in Rust, provide comprehensive error handling, and include extensive test coverage.

## Module Summaries

### 1. Math Module (`nekonomicon-core-math`)

**Purpose**: Provides mathematical operations for numeric calculations.

**Key Features**:
- Basic arithmetic (add, sub, mul, div, modulo)
- Rounding operations (round, floor, ceil, abs)
- Comparison and selection (compare, min, max, clamp)
- Advanced operations (pow, random)

**Location**: `packages/nekonomicon-core-math/`

**Example Operations**:
```rust
math::add("10", "5")        // "15"
math::compare("10", ">", "5") // "true"
math::random("1", "10")      // Random value between 1-10
```

**nekonomicon Script Usage**:
```spell
math add '10' '5' into @sum.
math compare '10' '>' '5' into @is_greater.
math clamp '15' min '0' max '10' into @clamped.
```

**Error Types**: Division by zero, invalid format, overflow

---

### 2. List Module (`nekonomicon-core-list`)

**Purpose**: Provides operations for ordered collections (arrays/lists).

**Key Features**:
- Adding elements (append, prepend)
- Removing elements (pop, shift)
- Inspecting elements (peek_end, peek_front, length)
- Stack (LIFO) and Queue (FIFO) operations

**Location**: `packages/nekonomicon-core-list/`

**Example Operations**:
```rust
let mut list = List::new();
list.append("apple");         // Add to end
list.prepend("banana");       // Add to start
list.pop()                    // Remove from end
list.shift()                  // Remove from start
```

**nekonomicon Script Usage**:
```spell
[] into ::items.
list append ::items 'apple'.
list pop ::items into @last.
list length ::items into @count.
```

**Error Types**: Empty list operations, invalid index

---

### 3. Text Module (`nekonomicon-core-text`)

**Purpose**: Provides string manipulation and pattern matching operations.

**Key Features**:
- Basic operations (concat, length, upper, lower, trim)
- Structural operations (split, join, slice, reverse)
- Pattern matching (matches, replace, replace_regex, capture)
- String inspection (starts_with, ends_with, contains)
- Formatting (repeat, pad_left, pad_right)

**Location**: `packages/nekonomicon-core-text/`

**Example Operations**:
```rust
text::concat(&["hello", " ", "world"])  // "hello world"
text::split("a,b,c", ",")               // ["a", "b", "c"]
text::matches("hello123", r"hello\d+")  // true
text::capture("ABC-123", r"([A-Z]+)-(\d+)") // ["ABC-123", "ABC", "123"]
```

**nekonomicon Script Usage**:
```spell
text concat 'hello' ' ' 'world' into @greeting.
text split @csv by ',' into ::parts.
text match @email '^[\w\.-]+@[\w\.-]+\.\w+$' into @is_valid.
```

**Error Types**: Invalid regex pattern, index out of bounds

---

### 4. HTTP Module (`nekonomicon-core-http`)

**Purpose**: Provides HTTP client functionality for web communication.

**Key Features**:
- HTTP methods (GET, POST, PUT, DELETE, PATCH)
- Request customization (headers, body)
- JSON support (post_json, put_json)
- Response handling (status, body, headers)

**Location**: `packages/nekonomicon-core-http/`

**Example Operations**:
```rust
http::get("https://api.example.com/users", None)
http::post_json("https://api.example.com/users", json_body, None)
http::delete("https://api.example.com/users/123", None)
```

**nekonomicon Script Usage**:
```spell
http get 'https://api.example.com/users' into ::response.
http post 'https://api.example.com/data'
  with body '{"name": "John"}'
  with headers '{"Content-Type": "application/json"}'
  into ::result.
```

**Error Types**: Request failed, invalid URL, timeout, network error

---

## Design Principles

All modules follow these design principles:

### 1. Safety First
- All operations return `Result` types for proper error handling
- Clear error types with descriptive messages
- No panics in normal operation
- Division by zero and other edge cases handled gracefully

### 2. String-Based Interface
- All operations accept and return strings (records in nekonomicon)
- Automatic type conversion (lensing) happens internally
- Consistent with nekonomicon's record-based type system

### 3. Comprehensive Testing
- Each module includes extensive unit tests
- Tests cover normal cases, edge cases, and error conditions
- All tests pass and provide good coverage

### 4. Clear Documentation
- Each module has a detailed README
- Inline documentation for all public functions
- Examples showing common usage patterns
- Links to nekonomicon language documentation

### 5. Minimal Dependencies
- Only essential external dependencies
- Well-maintained, widely-used crates
- No unnecessary complexity

## Building and Testing

### Build All Modules
```bash
cd /home/runner/work/nekonomicon/nekonomicon
cargo build --workspace
```

### Test All Modules
```bash
cargo test --workspace
```

### Build Individual Module
```bash
cd packages/nekonomicon-core-math
cargo build
cargo test
```

## Module Status

| Module | Status | Tests | Documentation |
|--------|--------|-------|---------------|
| Math   | ✅ Complete | ✅ 13 tests passing | ✅ README + inline docs |
| List   | ✅ Complete | ✅ 14 tests passing | ✅ README + inline docs |
| Text   | ✅ Complete | ✅ 16 tests passing | ✅ README + inline docs |
| HTTP   | ✅ Complete | ✅ 5 tests passing | ✅ README + inline docs |

## Integration with nekonomicon

These modules are designed to be called from nekonomicon scripts through the module system. The typical flow is:

1. **Script**: nekonomicon script uses module syntax (e.g., `math add '10' '5' into @sum.`)
2. **Parser**: nekonomicon parser recognizes module commands
3. **Runtime**: Runtime calls the appropriate Rust module function
4. **Module**: Module performs operation and returns result
5. **Runtime**: Result is stored in the specified variable/container

## Future Enhancements

Potential improvements for these modules:

### Math Module
- Expression evaluation with operator precedence
- Trigonometric functions (sin, cos, tan)
- Logarithmic functions
- Statistical operations (mean, median, mode)

### List Module
- Sorting operations
- Filtering and mapping
- Index-based operations (insert, remove at index)
- List slicing

### Text Module
- Advanced formatting (template strings)
- Character encoding operations
- More text analysis functions
- Locale-aware operations

### HTTP Module
- Async/await support
- WebSocket support
- Streaming responses
- Connection pooling
- More advanced authentication methods

## Contributing

When adding new modules or enhancing existing ones:

1. Follow the established patterns in existing modules
2. Include comprehensive unit tests
3. Write clear documentation (README + inline docs)
4. Ensure error handling is robust
5. Keep dependencies minimal
6. Follow Rust best practices

## Dependencies Summary

- **Math**: `rand` (random number generation)
- **List**: (no external dependencies)
- **Text**: `regex` (pattern matching)
- **HTTP**: `reqwest` (HTTP client), `serde`, `serde_json`

## License

All modules are part of the nekonomicon project. See the LICENSE file in the repository root for details.

## See Also

- [nekonomicon Documentation](docs/index.md)
- [Language Reference](docs/reference/)
- [Module Documentation](docs/modules/)
- Individual module README files in `packages/nekonomicon-core-*/README.md`
