use colored::Colorize;
use crate::utilities::modifier::handler;

/// This function handles the "help" command, which displays usage information
///
/// # Arguments
///
/// - `arguments` (`&[String]`) - Command-line arguments (currently unused)
///
/// # Examples
///
/// ```
/// use nekonomicon_core_cli::commands::command_help_handler;
///
/// command_help::command_help_handler(&[]);
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
    println!("  {} [command] [instructions]", "neko".bright_cyan());
    println!(
        "  {} {} with [command]",
        "neko".bright_cyan(),
        "help".bright_green(),
    );
    println!();

    println!("{}", "COMMANDS:".bright_yellow().bold());
    let commands_description = [
        ("story", "Get the version of the neko interpreter."),
        ("help", "Display help information."),
        ("conjure", "Run a nekonomicon script (.spell file)."),
        ("summon", "Install a nekonomicon module."),
        ("unsummon", "Uninstall a nekonomicon module."),
        ("grimoire", "List installed nekonomicon modules."),
        ("groom", "Validate/lint a nekonomicon script."),
        ("attune", "Format a nekonomicon script or directory."),
        ("brew", "Start the interactive nekonomicon REPL."),
    ];
    for (cmd, desc) in &commands_description {
        println!("  {:8}    {}", cmd.bright_green(), desc);
    }
    println!();

    println!("{}", "EXAMPLES:".bright_yellow().bold());
    println!(
        "  {} {} <script.spell>",
        "neko".bright_cyan(),
        "conjure".bright_green()
    );
    println!(
        "  {} {} cabinet text",
        "neko".bright_cyan(),
        "summon".bright_green()
    );
    println!(
        "{}",
        format!("Version: {}", env!("CARGO_PKG_VERSION")).dimmed()
    );
}
