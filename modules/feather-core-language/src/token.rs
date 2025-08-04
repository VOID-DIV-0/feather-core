pub struct Token {
    pub kind: TokenType,
    pub value: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Modifier, // as, is, with, etc.
    Variable, // @variable
    Interpolation, // ${variable}
    Literal, // 'content' 'content with spaces'
    Logical, // and, or, not, is, <, >, <=, >=
    Instruction, // module specific instructions
    Function, // #function
}


// Reserved Keywords
pub const LOGICAL_TOKEN_MODIFIER: [&str; 4] = [
    "as", "is", "with", "without",
];

pub const LOGICAL_TOKEN_KEYWORDS: [&str; 7] = [
    "and", "or", "not", "<", ">", "<=", ">="
];





