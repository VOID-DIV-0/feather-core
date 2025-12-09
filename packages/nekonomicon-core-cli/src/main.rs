use std::collections::HashMap;
use std::env;

mod commands;
use commands::story_handler;

// Define the type for your command handler
type CommandHandler = fn(&[String]);

fn help_handler(_args: &[String]) {
    println!("Available commands: story, help, conjure, summon, unsummon, grimoire, groom, attune, brew");
    println!();
    println!("Usage:");
    println!("  neko story [with style <brief|normal|full>]  - Show version information");
    println!("  neko help                                     - Show this help message");
    println!("  neko conjure <script.spell>                   - Run a nekonomicon script");
}


fn main() {
    // Create a mapping of command names to their handlers
    let mut commands: HashMap<&str, CommandHandler> = HashMap::new();
    commands.insert("story", story_handler);
    commands.insert("help", help_handler);

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure at least one command is provided
    if args.len() < 2 {
        eprintln!("No command provided. Use 'neko help' for usage information.");
        std::process::exit(1);
    }

    let command = args[1].as_str();
    let rest = &args[2..];

    match commands.get(command) {
        Some(handler) => handler(rest),
        None => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}