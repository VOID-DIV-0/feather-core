use colored::Colorize;

/// This function handles the "help" command, which displays usage information
///
/// # Arguments
///
/// - `arguments` (`&[String]`) - Command-line arguments (currently unused)
///
/// # Examples
///
/// ```
/// use nekonomicon_core_cli::commands::command_help;
///
/// command_help::handler(&[]);
/// ```
pub fn command_help_handler(arguments: &[String]) {
    let _ = arguments; // Currently unused

    println!("{}", "╭─────────────────────────╮".bright_magenta());
    println!("{}", "│                         │".bright_magenta());
    println!("{}", "│ ▖ ▖  ▌          ▘       │".bright_magenta());
    println!("{}", "│ ▛▖▌█▌▙▘▛▌▛▌▛▌▛▛▌▌▛▘▛▌▛▌ │".bright_magenta());
    println!("{}", "│ ▌▝▌▙▖▛▖▙▌▌▌▙▌▌▌▌▌▙▖▙▌▌▌ │".bright_magenta());
    println!("{}", "│                         │".bright_magenta());
    println!("{}", "╰─────────────────── CLI ─╯".bright_magenta());
    println!();

    println!("{}", "USAGE:".bright_yellow().bold());
    println!("  {} <script.spell>", "neko".bright_white());
    println!("  {} [command] [options]", "neko".bright_white());
    println!();

    println!("{}", "COMMANDS:".bright_yellow().bold());
    println!(
        "  {}    Display version information",
        "story".bright_green()
    );
    println!("  {}     Show this help message", "help".bright_green());
    println!();

    println!("{}", "EXAMPLES:".bright_yellow().bold());
    println!("  {} hello_world.spell", "neko".bright_white());
    println!("  {} story with style full", "neko".bright_white());
    println!();

    println!(
        "{}",
        format!("Version: {}", env!("CARGO_PKG_VERSION")).dimmed()
    );
}
