use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "grammar/nekonomicon.pest"]
pub struct NekonomiconParser;

/// Simple AST for minimal nekonomicon parser
#[derive(Debug, Clone)]
pub enum Command {
    Say { text: String },
}

pub fn parse(input: &str) -> Result<Command, Box<dyn std::error::Error>> {
    let parsed = NekonomiconParser::parse(Rule::command, input)?
        .next()
        .ok_or("No command found")?;
    
    parse_command(parsed)
}

fn parse_command(pair: Pair<Rule>) -> Result<Command, Box<dyn std::error::Error>> {
    match pair.as_rule() {
        Rule::command => {
            // Get the say_command inside
            let say_command = pair.into_inner().next().ok_or("No say command found")?;
            parse_say_command(say_command)
        }
        _ => Err(format!("Expected command, found {:?}", pair.as_rule()).into()),
    }
}

fn parse_say_command(pair: Pair<Rule>) -> Result<Command, Box<dyn std::error::Error>> {
    match pair.as_rule() {
        Rule::say_command => {
            let mut inner = pair.into_inner();
            
            // Skip "say" keyword, get string_literal
            let string_literal = inner.next().ok_or("No string literal found")?;
            let text = parse_string_literal(string_literal)?;
            
            Ok(Command::Say { text })
        }
        _ => Err(format!("Expected say_command, found {:?}", pair.as_rule()).into()),
    }
}

fn parse_string_literal(pair: Pair<Rule>) -> Result<String, Box<dyn std::error::Error>> {
    match pair.as_rule() {
        Rule::string_literal => {
            let content = pair.as_str();
            // Remove surrounding quotes and handle basic escape sequences
            let without_quotes = &content[1..content.len()-1];
            let unescaped = without_quotes.replace("\\'", "'").replace("\\\\", "\\");
            Ok(unescaped)
        }
        _ => Err(format!("Expected string_literal, found {:?}", pair.as_rule()).into()),
    }
}

pub fn execute(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::Say { text } => {
            println!("{}", text);
            Ok(())
        }
    }
}