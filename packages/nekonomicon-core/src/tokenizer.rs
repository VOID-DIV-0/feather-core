use crate::token::{Token, TokenType, LOGICAL_TOKEN_KEYWORDS, LOGICAL_TOKEN_MODIFIER };

// Tokenizes the input string into a vector of tokens.
// This function handles keywords, variables, string literals,
// and comments, returning a vector of `Token` structs.
pub fn tokenize(input: &str) -> Vec<Token> {
    use TokenType::*;

    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.peek().copied() {
        match c {
            ' ' | '\t' | '\n' => {
                skip_whitespace(&mut chars);
            }
            '~' => {
                skip_comment_until_end(&mut chars);
            }
            '\'' => {
                // Parse string literal
                chars.next(); // Skip starting quote
                let mut literal = String::new();
                while let Some(next) = chars.next() {
                    if next == '\'' {
                        break;
                    }
                    literal.push(next);
                }
                tokens.push(Token { kind: Literal, value: literal });
            }
            '@' => {
                // Parse variable
                chars.next(); // skip '@'
                let mut name = String::new();
                
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        name.push(ch);
                        chars.next();
                    } else if ch == '{' {
                        // Handle variable with hint
                        chars.next(); // Skip '{'
                        let mut hint = String::new();
                        while let Some(next) = chars.next() {
                            if next == '}' {
                                break; // End of hint
                            }
                            hint.push(next);
                        }
                        tokens.push(Token { kind: Interpolation, value: hint });
                        break; // Exit variable parsing
                    } else {
                        break; // End of variable name
                    }
                }

                tokens.push(Token { kind: Variable, value: name});
            }
            _ => {
                // Parse keyword/command/identifier
                let mut word = String::new();

                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        word.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let kind = if LOGICAL_TOKEN_MODIFIER.contains(&word.as_str()) {
                    TokenType::Modifier
                } else if LOGICAL_TOKEN_KEYWORDS.contains(&word.as_str()) {
                    TokenType::Logical
                } else {
                    TokenType::Instruction // Treat as a command or instruction
                };

                tokens.push(Token { kind, value: word });
            }
        }
    }

    tokens
}

// Helper functions to skip whitespace and empty lines.
fn skip_whitespace(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) {
    chars.next();
}


fn skip_comment_until_end(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) {
    chars.next();
    // Skip '~'

    // This is a comment, skip until newline
    while let Some(&ch) = chars.peek() {
      chars.next();
        if ch == '\n' {
            break; // End of comment
        }
    }
}