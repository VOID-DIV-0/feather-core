#[cfg(test)]
mod tests {
    use feather_core_language::tokenize; // Import the tokenize function
    use feather_core_language::token::{TokenType::*};

    #[test]
    fn tokenize_skips_comment_at_end_of_line() {
        // Arrange & Act
        let tokens = tokenize("say 'hi' ~this is a comment");

        // Assert
        assert!(tokens.iter().any(|t| t.kind == Instruction && t.value == "say"));
        assert!(tokens.iter().any(|t| t.kind == Literal && t.value == "hi"));
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn tokenize_skips_comment_in_middle_of_line() {
        // Arrange & Act
        let tokens = tokenize("say ~this is a comment 'hi'");

        // Assert
        assert!(tokens.iter().any(|t| t.kind == Instruction && t.value == "say"));
        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn tokenize_line_with_only_comment_results_in_no_tokens() {
        // Arrange & Act
        let tokens = tokenize("~this is a comment say 'hi'");

        // Assert
        assert_eq!(tokens.len(), 0);
    }
}