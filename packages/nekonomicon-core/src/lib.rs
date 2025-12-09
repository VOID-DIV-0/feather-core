pub mod parser;

pub use parser::{Command, parse, execute};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_say() {
        let result = parse("say 'Hello, World!'.");
        match result {
            Ok(Command::Say { text }) => {
                assert_eq!(text, "Hello, World!");
            }
            Err(e) => {
                panic!("Parse failed: {}", e);
            }
        }
    }

    #[test]
    fn test_say_with_quotes() {
        let result = parse("say 'I\\'m learning nekonomicon!'.");
        match result {
            Ok(Command::Say { text }) => {
                assert_eq!(text, "I'm learning nekonomicon!");
            }
            Err(e) => {
                panic!("Parse failed: {}", e);
            }
        }
    }
}




