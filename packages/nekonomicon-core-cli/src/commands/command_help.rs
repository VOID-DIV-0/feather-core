use colored::Colorize;
use std::collections::HashMap;

type HelpFunction = fn();

/// This function handles the "help" command, which displays usage information
///
/// # Arguments
///
/// - `arguments` (`&[String]`) - Command-line arguments for specific command help
///
/// # Examples
///
/// ```
/// use nekonomicon_core_cli::commands::command_help_handler;
///
/// // General help
/// command_help::command_help_handler(&[]);
///
/// // Specific command help
/// command_help::command_help_handler(&["conjure".to_string()]);
/// ```
pub fn command_help_handler(arguments: &[String]) {
    // Map commands to their help functions
    let help_map: HashMap<&str, HelpFunction> = HashMap::from([
        ("story", print_story_help as HelpFunction),
        ("help", print_help_help as HelpFunction),
        ("conjure", print_conjure_help as HelpFunction),
        ("summon", print_summon_help as HelpFunction),
        ("unsummon", print_unsummon_help as HelpFunction),
        ("grimoire", print_grimoire_help as HelpFunction),
        ("groom", print_groom_help as HelpFunction),
        ("attune", print_attune_help as HelpFunction),
        ("brew", print_brew_help as HelpFunction),
    ]);

    // If a specific command is requested
    if let Some(cmd) = arguments.first() {
        if let Some(help_fn) = help_map.get(cmd.as_str()) {
            help_fn(); // Call the mapped function
            return;
        } else {
            println!("{} Unknown command: {}", "Error:".bright_red().bold(), cmd);
            println!(
                "Run {} for a list of available commands.",
                "neko help".bright_cyan()
            );
            return;
        }
    }

    // Otherwise show general help
    print_general_help();
}

fn print_general_help() {
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
        "  {} {} [command]",
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
        println!("  {:10}  {}", cmd.bright_green(), desc);
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
        "  {} {} conjure",
        "neko".bright_cyan(),
        "help".bright_green()
    );
    println!();
    println!(
        "{}",
        format!("Version: {}", env!("CARGO_PKG_VERSION")).dimmed()
    );
}

fn print_story_help() {
    println!("{}", "╭─────────────────────────╮".bright_magenta());
    println!("{}", "│ COMMAND: story          │".bright_magenta());
    println!("{}", "╰─────────────────────────╯".bright_magenta());
    println!();
    println!("{}", "DESCRIPTION:".bright_yellow().bold());
    println!("  Get the version of the neko interpreter.");
    println!();
    println!("{}", "USAGE:".bright_yellow().bold());
    println!("  {} {}", "neko".bright_cyan(), "story".bright_green());
    println!();
    println!("{}", "EXAMPLES:".bright_yellow().bold());
    println!("  {} {}", "neko".bright_cyan(), "story".bright_green());
}

fn print_help_help() {
    println!("{}", "╭─────────────────────────╮".bright_magenta());
    println!("{}", "│ COMMAND: help           │".bright_magenta());
    println!("{}", "╰─────────────────────────╯".bright_magenta());
    println!();
    println!("{}", "DESCRIPTION:".bright_yellow().bold());
    println!("  Display help information for nekonomicon commands.");
    println!();
    println!("{}", "USAGE:".bright_yellow().bold());
    println!("  {} {}", "neko".bright_cyan(), "help".bright_green());
    println!(
        "  {} {} [command]",
        "neko".bright_cyan(),
        "help".bright_green()
    );
    println!();
    println!("{}", "EXAMPLES:".bright_yellow().bold());
    println!(
        "  {} {}              {}",
        "neko".bright_cyan(),
        "help".bright_green(),
        "# Show all commands".dimmed()
    );
    println!(
        "  {} {} conjure      {}",
        "neko".bright_cyan(),
        "help".bright_green(),
        "# Show help for conjure command".dimmed()
    );
    println!(
        "  {} {} summon       {}",
        "neko".bright_cyan(),
        "help".bright_green(),
        "# Show help for summon command".dimmed()
    );
}

fn print_conjure_help() {
    println!("{}", "╭─────────────────────────╮".bright_magenta());
    println!("{}", "│ COMMAND: conjure        │".bright_magenta());
    println!("{}", "╰─────────────────────────╯".bright_magenta());
    println!();
    println!("{}", "DESCRIPTION:".bright_yellow().bold());
    println!("  Run a nekonomicon script (.spell file).");
    println!();
    println!("{}", "USAGE:".bright_yellow().bold());
    println!(
        "  {} {} <script.spell> [args]",
        "neko".bright_cyan(),
        "conjure".bright_green()
    );
    println!();
    println!("{}", "ARGUMENTS:".bright_yellow().bold());
    println!(
        "  {}  Path to the .spell file to execute",
        "<script.spell>".bright_blue()
    );
    println!(
        "  {}        Optional arguments passed to the script",
        "[args]".bright_blue()
    );
    println!();
    println!("{}", "EXAMPLES:".bright_yellow().bold());
    println!(
        "  {} {} deploy.spell",
        "neko".bright_cyan(),
        "conjure".bright_green()
    );
    println!(
        "  {} {} script.spell --debug",
        "neko".bright_cyan(),
        "conjure".bright_green()
    );
    println!(
        "  {} {} automation.spell arg1 arg2",
        "neko".bright_cyan(),
        "conjure".bright_green()
    );
}

fn print_summon_help() {
    println!("{}", "╭─────────────────────────╮".bright_magenta());
    println!("{}", "│ COMMAND: summon         │".bright_magenta());
    println!("{}", "╰─────────────────────────╯".bright_magenta());
    println!();
    println!("{}", "DESCRIPTION:".bright_yellow().bold());
    println!("  Install nekonomicon modules globally.");
    println!();
    println!("{}", "USAGE:".bright_yellow().bold());
    println!(
        "  {} {} <module> [module2 ...]",
        "neko".bright_cyan(),
        "summon".bright_green()
    );
    println!(
        "  {} {} <module>:<version>",
        "neko".bright_cyan(),
        "summon".bright_green()
    );
    println!();
    println!("{}", "ARGUMENTS:".bright_yellow().bold());
    println!(
        "  {}    Name of the module to install",
        "<module>".bright_blue()
    );
    println!(
        "  {}  Specific version (for external modules)",
        "<version>".bright_blue()
    );
    println!();
    println!("{}", "MODULE TYPES:".bright_yellow().bold());
    println!(
        "  {}  Built-in standard library modules (versionless)",
        "STD Modules".bright_green()
    );
    println!("  {}      cabinet, vault, network, text, math, etc.");
    println!();
    println!(
        "  {}      Third-party modules (require version)",
        "External Modules".bright_green()
    );
    println!("  {}      mymodule:1.2.3, company/auth:2.0.0");
    println!();
    println!("{}", "EXAMPLES:".bright_yellow().bold());
    println!(
        "  {} {} cabinet",
        "neko".bright_cyan(),
        "summon".bright_green()
    );
    println!(
        "  {} {} cabinet vault network",
        "neko".bright_cyan(),
        "summon".bright_green()
    );
    println!(
        "  {} {} mymodule:1.2.3",
        "neko".bright_cyan(),
        "summon".bright_green()
    );
    println!(
        "  {} {} company/auth:latest",
        "neko".bright_cyan(),
        "summon".bright_green()
    );
}

fn print_unsummon_help() {
    println!("{}", "╭─────────────────────────╮".bright_magenta());
    println!("{}", "│ COMMAND: unsummon       │".bright_magenta());
    println!("{}", "╰─────────────────────────╯".bright_magenta());
    println!();
    println!("{}", "DESCRIPTION:".bright_yellow().bold());
    println!("  Uninstall nekonomicon modules.");
    println!();
    println!("{}", "USAGE:".bright_yellow().bold());
    println!(
        "  {} {} <module> [module2 ...]",
        "neko".bright_cyan(),
        "unsummon".bright_green()
    );
    println!();
    println!("{}", "ARGUMENTS:".bright_yellow().bold());
    println!(
        "  {}    Name of the module to uninstall",
        "<module>".bright_blue()
    );
    println!();
    println!("{}", "EXAMPLES:".bright_yellow().bold());
    println!(
        "  {} {} cabinet",
        "neko".bright_cyan(),
        "unsummon".bright_green()
    );
    println!(
        "  {} {} vault network",
        "neko".bright_cyan(),
        "unsummon".bright_green()
    );
}

fn print_grimoire_help() {
    println!("{}", "╭─────────────────────────╮".bright_magenta());
    println!("{}", "│ COMMAND: grimoire       │".bright_magenta());
    println!("{}", "╰─────────────────────────╯".bright_magenta());
    println!();
    println!("{}", "DESCRIPTION:".bright_yellow().bold());
    println!("  List all installed nekonomicon modules.");
    println!();
    println!("{}", "USAGE:".bright_yellow().bold());
    println!("  {} {}", "neko".bright_cyan(), "grimoire".bright_green());
    println!();
    println!("{}", "OUTPUT:".bright_yellow().bold());
    println!("  Displays installed modules with their versions and types.");
    println!();
    println!("{}", "EXAMPLES:".bright_yellow().bold());
    println!("  {} {}", "neko".bright_cyan(), "grimoire".bright_green());
}

fn print_groom_help() {
    println!("{}", "╭─────────────────────────╮".bright_magenta());
    println!("{}", "│ COMMAND: groom          │".bright_magenta());
    println!("{}", "╰─────────────────────────╯".bright_magenta());
    println!();
    println!("{}", "DESCRIPTION:".bright_yellow().bold());
    println!("  Validate and lint a nekonomicon script.");
    println!();
    println!("{}", "USAGE:".bright_yellow().bold());
    println!(
        "  {} {} <script.spell>",
        "neko".bright_cyan(),
        "groom".bright_green()
    );
    println!();
    println!("{}", "ARGUMENTS:".bright_yellow().bold());
    println!(
        "  {}  Path to the .spell file to validate",
        "<script.spell>".bright_blue()
    );
    println!();
    println!("{}", "OUTPUT:".bright_yellow().bold());
    println!("  Reports syntax errors, warnings, and style issues.");
    println!();
    println!("{}", "EXAMPLES:".bright_yellow().bold());
    println!(
        "  {} {} script.spell",
        "neko".bright_cyan(),
        "groom".bright_green()
    );
    println!(
        "  {} {} automation.spell",
        "neko".bright_cyan(),
        "groom".bright_green()
    );
}

fn print_attune_help() {
    println!("{}", "╭─────────────────────────╮".bright_magenta());
    println!("{}", "│ COMMAND: attune         │".bright_magenta());
    println!("{}", "╰─────────────────────────╯".bright_magenta());
    println!();
    println!("{}", "DESCRIPTION:".bright_yellow().bold());
    println!("  Format a nekonomicon script or directory according to style guide.");
    println!();
    println!("{}", "USAGE:".bright_yellow().bold());
    println!(
        "  {} {} <script.spell>",
        "neko".bright_cyan(),
        "attune".bright_green()
    );
    println!(
        "  {} {} <directory>",
        "neko".bright_cyan(),
        "attune".bright_green()
    );
    println!();
    println!("{}", "ARGUMENTS:".bright_yellow().bold());
    println!(
        "  {}  Path to file or directory to format",
        "<path>".bright_blue()
    );
    println!();
    println!("{}", "BEHAVIOR:".bright_yellow().bold());
    println!("  Automatically formats code according to nekonomicon style guide.");
    println!("  Formats all .spell files in directory recursively.");
    println!();
    println!("{}", "EXAMPLES:".bright_yellow().bold());
    println!(
        "  {} {} script.spell",
        "neko".bright_cyan(),
        "attune".bright_green()
    );
    println!(
        "  {} {} ./spells",
        "neko".bright_cyan(),
        "attune".bright_green()
    );
}

fn print_brew_help() {
    println!("{}", "╭─────────────────────────╮".bright_magenta());
    println!("{}", "│ COMMAND: brew           │".bright_magenta());
    println!("{}", "╰─────────────────────────╯".bright_magenta());
    println!();
    println!("{}", "DESCRIPTION:".bright_yellow().bold());
    println!("  Start the interactive nekonomicon REPL (Read-Eval-Print Loop).");
    println!();
    println!("{}", "USAGE:".bright_yellow().bold());
    println!("  {} {}", "neko".bright_cyan(), "brew".bright_green());
    println!();
    println!("{}", "FEATURES:".bright_yellow().bold());
    println!("  • Execute nekonomicon commands interactively");
    println!("  • Test code snippets without creating files");
    println!("  • Explore modules and experiment with syntax");
    println!("  • View command history and results");
    println!();
    println!("{}", "EXAMPLES:".bright_yellow().bold());
    println!("  {} {}", "neko".bright_cyan(), "brew".bright_green());
    println!();
    println!("{}", "REPL COMMANDS:".bright_yellow().bold());
    println!("  {}      Exit the REPL", ".exit".bright_blue());
    println!("  {}      Show REPL help", ".help".bright_blue());
    println!("  {}     Clear screen", ".clear".bright_blue());
}
