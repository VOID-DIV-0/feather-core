//! # List Module for Nekonomicon
//!
//! List manipulation operations for ordered collections.
//! Lists support adding, removing, and inspecting elements from both ends,
//! making them suitable for stack and queue operations.
//!
//! All list operations work with String values stored in the list.

use std::error::Error;
use std::fmt;

/// Error types for list operations
#[derive(Debug)]
pub enum ListError {
    /// Cannot perform operation on empty list
    EmptyList(String),
    /// Invalid index access
    InvalidIndex(usize),
    /// List not found
    NotFound(String),
}

impl fmt::Display for ListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListError::EmptyList(op) => write!(f, "LIST-001: Cannot {} from empty list", op),
            ListError::InvalidIndex(idx) => write!(f, "LIST-002: Invalid index: {}", idx),
            ListError::NotFound(name) => write!(f, "LIST-003: List not found: {}", name),
        }
    }
}

impl Error for ListError {}

/// Represents a List container
#[derive(Debug, Clone)]
pub struct List {
    items: Vec<String>,
}

impl List {
    /// Create a new empty list
    pub fn new() -> Self {
        List { items: Vec::new() }
    }

    /// Create a list from existing items
    pub fn from_vec(items: Vec<String>) -> Self {
        List { items }
    }

    /// Add element to end of list
    pub fn append(&mut self, value: String) {
        self.items.push(value);
    }

    /// Add element to beginning of list
    pub fn prepend(&mut self, value: String) {
        self.items.insert(0, value);
    }

    /// Remove and return last element
    pub fn pop(&mut self) -> Result<String, ListError> {
        self.items
            .pop()
            .ok_or_else(|| ListError::EmptyList("pop".to_string()))
    }

    /// Remove and return first element
    pub fn shift(&mut self) -> Result<String, ListError> {
        if self.items.is_empty() {
            return Err(ListError::EmptyList("shift".to_string()));
        }
        Ok(self.items.remove(0))
    }

    /// View last element without removal
    pub fn peek_end(&self) -> Result<String, ListError> {
        self.items
            .last()
            .cloned()
            .ok_or_else(|| ListError::EmptyList("peek".to_string()))
    }

    /// View first element without removal
    pub fn peek_front(&self) -> Result<String, ListError> {
        self.items
            .first()
            .cloned()
            .ok_or_else(|| ListError::EmptyList("peek".to_string()))
    }

    /// Get number of elements
    pub fn length(&self) -> usize {
        self.items.len()
    }

    /// Check if list is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get element at index (0-based)
    pub fn get(&self, index: usize) -> Result<String, ListError> {
        self.items
            .get(index)
            .cloned()
            .ok_or_else(|| ListError::InvalidIndex(index))
    }

    /// Get all items as a vector
    pub fn items(&self) -> &Vec<String> {
        &self.items
    }

    /// Clear all elements
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Convert list to formatted string representation
    pub fn to_string(&self) -> String {
        format!("[{}]", self.items.join(", "))
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

/// Public API functions for list operations

/// Append element to list
pub fn append(list: &mut List, value: String) {
    list.append(value);
}

/// Prepend element to list
pub fn prepend(list: &mut List, value: String) {
    list.prepend(value);
}

/// Pop element from end of list
pub fn pop(list: &mut List) -> Result<String, ListError> {
    list.pop()
}

/// Shift element from front of list
pub fn shift(list: &mut List) -> Result<String, ListError> {
    list.shift()
}

/// Peek at end element
pub fn peek_end(list: &List) -> Result<String, ListError> {
    list.peek_end()
}

/// Peek at front element
pub fn peek_front(list: &List) -> Result<String, ListError> {
    list.peek_front()
}

/// Get list length
pub fn length(list: &List) -> usize {
    list.length()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list() {
        let list = List::new();
        assert_eq!(list.length(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_append() {
        let mut list = List::new();
        list.append("apple".to_string());
        list.append("banana".to_string());
        list.append("cherry".to_string());
        
        assert_eq!(list.length(), 3);
        assert_eq!(list.get(0).unwrap(), "apple");
        assert_eq!(list.get(2).unwrap(), "cherry");
    }

    #[test]
    fn test_prepend() {
        let mut list = List::new();
        list.prepend("first".to_string());
        list.prepend("second".to_string());
        
        assert_eq!(list.length(), 2);
        assert_eq!(list.get(0).unwrap(), "second");
        assert_eq!(list.get(1).unwrap(), "first");
    }

    #[test]
    fn test_pop() {
        let mut list = List::new();
        list.append("a".to_string());
        list.append("b".to_string());
        list.append("c".to_string());
        
        assert_eq!(list.pop().unwrap(), "c");
        assert_eq!(list.length(), 2);
        assert_eq!(list.pop().unwrap(), "b");
        assert_eq!(list.pop().unwrap(), "a");
        assert!(list.pop().is_err());
    }

    #[test]
    fn test_shift() {
        let mut list = List::new();
        list.append("first".to_string());
        list.append("second".to_string());
        list.append("third".to_string());
        
        assert_eq!(list.shift().unwrap(), "first");
        assert_eq!(list.length(), 2);
        assert_eq!(list.shift().unwrap(), "second");
        assert_eq!(list.shift().unwrap(), "third");
        assert!(list.shift().is_err());
    }

    #[test]
    fn test_peek_end() {
        let mut list = List::new();
        list.append("one".to_string());
        list.append("two".to_string());
        list.append("three".to_string());
        
        assert_eq!(list.peek_end().unwrap(), "three");
        assert_eq!(list.length(), 3); // List unchanged
    }

    #[test]
    fn test_peek_front() {
        let mut list = List::new();
        list.append("one".to_string());
        list.append("two".to_string());
        list.append("three".to_string());
        
        assert_eq!(list.peek_front().unwrap(), "one");
        assert_eq!(list.length(), 3); // List unchanged
    }

    #[test]
    fn test_peek_empty_list() {
        let list = List::new();
        assert!(list.peek_end().is_err());
        assert!(list.peek_front().is_err());
    }

    #[test]
    fn test_length() {
        let mut list = List::new();
        assert_eq!(list.length(), 0);
        
        list.append("item".to_string());
        assert_eq!(list.length(), 1);
        
        list.append("another".to_string());
        assert_eq!(list.length(), 2);
        
        list.pop().unwrap();
        assert_eq!(list.length(), 1);
    }

    #[test]
    fn test_get_invalid_index() {
        let mut list = List::new();
        list.append("only".to_string());
        
        assert!(list.get(0).is_ok());
        assert!(list.get(1).is_err());
        assert!(list.get(10).is_err());
    }

    #[test]
    fn test_clear() {
        let mut list = List::new();
        list.append("a".to_string());
        list.append("b".to_string());
        
        list.clear();
        assert_eq!(list.length(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_to_string() {
        let mut list = List::new();
        list.append("apple".to_string());
        list.append("banana".to_string());
        
        let str_repr = list.to_string();
        assert!(str_repr.contains("apple"));
        assert!(str_repr.contains("banana"));
    }

    #[test]
    fn test_stack_behavior() {
        // LIFO: Last In First Out
        let mut stack = List::new();
        stack.append("1".to_string());
        stack.append("2".to_string());
        stack.append("3".to_string());
        
        assert_eq!(stack.pop().unwrap(), "3");
        assert_eq!(stack.pop().unwrap(), "2");
        assert_eq!(stack.pop().unwrap(), "1");
    }

    #[test]
    fn test_queue_behavior() {
        // FIFO: First In First Out
        let mut queue = List::new();
        queue.append("1".to_string());
        queue.append("2".to_string());
        queue.append("3".to_string());
        
        assert_eq!(queue.shift().unwrap(), "1");
        assert_eq!(queue.shift().unwrap(), "2");
        assert_eq!(queue.shift().unwrap(), "3");
    }
}
