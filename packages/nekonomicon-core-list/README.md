# nekonomicon-core-list

List manipulation operations for the nekonomicon scripting language.

## Overview

The List module provides operations for working with ordered collections of values. Lists support adding, removing, and inspecting elements from both ends, making them suitable for both stack (LIFO) and queue (FIFO) operations.

## Features

### Adding Elements
- **append**: Add element to end of list
- **prepend**: Add element to beginning of list

### Removing Elements
- **pop**: Remove and return last element
- **shift**: Remove and return first element

### Inspecting Elements
- **peek_end**: View last element without removal
- **peek_front**: View first element without removal
- **length**: Get number of elements
- **get**: Access element at specific index

### Other Operations
- **clear**: Remove all elements
- **is_empty**: Check if list is empty
- **to_string**: Get string representation

## Usage Example

```rust
use nekonomicon_core_list::*;

// Create a new list
let mut list = List::new();

// Add elements
list.append("apple".to_string());
list.append("banana".to_string());
list.prepend("cherry".to_string());

// Access elements
let first = list.peek_front().unwrap(); // "cherry"
let last = list.peek_end().unwrap();    // "banana"
let count = list.length();              // 3

// Remove elements
let removed = list.pop().unwrap();      // "banana"
let removed = list.shift().unwrap();    // "cherry"

// Stack behavior (LIFO)
let mut stack = List::new();
stack.append("1".to_string());
stack.append("2".to_string());
let value = stack.pop().unwrap(); // "2"

// Queue behavior (FIFO)
let mut queue = List::new();
queue.append("1".to_string());
queue.append("2".to_string());
let value = queue.shift().unwrap(); // "1"
```

## Error Handling

The module defines several error types:
- **LIST-001**: Cannot perform operation on empty list
- **LIST-002**: Invalid index access
- **LIST-003**: List not found

Operations that might fail return `Result<String, ListError>`.

## Common Patterns

### Stack (LIFO - Last In First Out)
Use `append` to push and `pop` to get the last element added:
```rust
let mut stack = List::new();
stack.append("item1".to_string());
stack.append("item2".to_string());
let last = stack.pop().unwrap(); // "item2"
```

### Queue (FIFO - First In First Out)
Use `append` to enqueue and `shift` to get the first element added:
```rust
let mut queue = List::new();
queue.append("item1".to_string());
queue.append("item2".to_string());
let first = queue.shift().unwrap(); // "item1"
```

## Testing

Run tests with:
```bash
cargo test
```

Tests cover:
- Basic operations (append, prepend, pop, shift)
- Peek operations
- Stack and queue behaviors
- Error conditions (empty list operations)
- Edge cases

## nekonomicon Script Usage

In nekonomicon scripts, the list module is used as:

```spell
[] into ::items.
list append ::items 'apple'.
list pop ::items into @last.
list peek ::items end into @value.
list length ::items into @count.
```

See the [List module documentation](../../docs/modules/data/list.md) for complete nekonomicon language syntax and examples.

## Best Practices

- Use `safe` clause when operating on lists that might be empty
- Check list length before operations if emptiness is uncertain
- Use `peek` when you need to inspect without modifying the list
- Prefer explicit list initialization with `[]` for clarity

## License

Part of the nekonomicon project. See LICENSE file for details.
