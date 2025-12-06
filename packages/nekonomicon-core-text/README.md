# nekonomicon-core-text

String and pattern operations for the nekonomicon scripting language.

## Overview

The Text module provides comprehensive string manipulation capabilities, including:
- Basic operations (concatenation, length, case conversion)
- Structural operations (split, join, slice)
- Pattern matching and replacement (regex support)
- String inspection (starts with, ends with, contains)
- Formatting operations (padding, trimming)

## Features

### Basic Operations
- **concat**: Concatenate multiple strings
- **length**: Get character count
- **upper**: Convert to uppercase
- **lower**: Convert to lowercase
- **trim**: Remove whitespace from ends

### Structural Operations
- **split**: Split string by delimiter
- **join**: Join strings with separator
- **slice**: Extract substring by indices
- **reverse**: Reverse string

### Pattern Matching
- **matches**: Check if string matches regex pattern
- **replace**: Replace all occurrences
- **replace_regex**: Replace using regex pattern
- **capture**: Extract groups from regex match

### String Inspection
- **starts_with**: Check prefix
- **ends_with**: Check suffix
- **contains**: Check substring presence
- **compare**: Compare strings (case-sensitive or insensitive)

### Formatting
- **repeat**: Repeat string n times
- **pad_left**: Add left padding
- **pad_right**: Add right padding
- **trim_start**: Remove leading whitespace
- **trim_end**: Remove trailing whitespace

## Usage Example

```rust
use nekonomicon_core_text::*;

// Basic operations
let result = concat(&["hello", " ", "world"]); // "hello world"
let len = length("hello");                      // 5
let upper = upper("hello");                     // "HELLO"

// Splitting and joining
let parts = split("a,b,c", ",");                // ["a", "b", "c"]
let joined = join(&parts, "-");                 // "a-b-c"

// Pattern matching
let is_match = matches("hello123", r"hello\d+").unwrap(); // true
let replaced = replace("hello world", "world", "rust");   // "hello rust"

// Capture groups
let captures = capture("ABC-123", r"([A-Z]+)-(\d+)").unwrap();
// captures[0] = "ABC-123"
// captures[1] = "ABC"
// captures[2] = "123"

// String inspection
let has_prefix = starts_with("hello", "hel");  // true
let has_suffix = ends_with("hello", "lo");     // true
let has_substr = contains("hello", "ll");      // true

// Formatting
let repeated = repeat("ab", 3);                // "ababab"
let padded = pad_left("5", 3, '0');           // "005"
```

## Error Handling

The module defines several error types:
- **TEXT-001**: Invalid regex pattern
- **TEXT-002**: Index out of bounds
- **TEXT-003**: Invalid format

Operations that might fail return `Result<String, TextError>` or `Result<Vec<String>, TextError>`.

## Regular Expression Support

The text module uses the `regex` crate for pattern matching operations. Supported features include:
- Character classes and ranges
- Quantifiers (*, +, ?, {n}, {n,m})
- Groups and capturing
- Anchors (^, $)
- Alternation (|)
- Unicode support

Example patterns:
```rust
// Email validation
matches("test@example.com", r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();

// Extract numbers
capture("Price: $123.45", r"\$(\d+\.\d+)").unwrap();

// Replace multiple spaces
replace_regex("hello    world", r"\s+", " ").unwrap();
```

## Testing

Run tests with:
```bash
cargo test
```

Tests cover:
- All basic operations
- Pattern matching and regex
- Edge cases (empty strings, Unicode)
- Error conditions

## nekonomicon Script Usage

In nekonomicon scripts, the text module is used as:

```spell
text concat 'hello' ' ' 'world' into @greeting.
text split @csv by ',' into ::parts.
text upper @name into @name_upper.
text match @email '^[\w\.-]+@[\w\.-]+\.\w+$' into @is_valid.
text replace @text 'old' 'new' into @updated.
```

See the [Text module documentation](../../docs/modules/text/text.md) for complete nekonomicon language syntax and examples.

## Best Practices

- Use regex operations for complex pattern matching
- Always handle potential regex compilation errors
- Consider Unicode character boundaries when slicing
- Use `trim` before comparisons to avoid whitespace issues
- Prefer case-insensitive comparison when user input is involved

## Performance Notes

- String concatenation with `concat` is efficient for small numbers of strings
- For large-scale concatenation, consider collecting into a container first
- Regex compilation has overhead; cache compiled patterns if possible
- Unicode-aware operations (length, slice) may be slower than byte operations

## License

Part of the nekonomicon project. See LICENSE file for details.
