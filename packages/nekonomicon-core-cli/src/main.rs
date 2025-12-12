use std::collections::HashMap;
use std::env;

mod commands;

use crate::commands::command_help::command_help_handler;
use crate::commands::command_story::command_story_handler;

// Define the type for your command handler
type CommandHandler = fn(&[String]);

fn main() {
    // Create a mapping of command names to their handlers
    let mut commands: HashMap<&str, CommandHandler> = HashMap::new();
    commands.insert("story", command_story_handler);
    commands.insert("help", command_help_handler);

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
