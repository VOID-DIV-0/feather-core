/// This function handles the "story" command, which outputs the version information
///
/// # Arguments
///
/// - `args` (`&[String]`) - Describe this parameter.
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let _ = command_help_handler();
/// ```
pub fn command_story_handler(arguments: &[String]) {
    // Default format
    let mut style = "normal";

    // Parse for "with style <value>"
    let mut index = 0;

    while index < arguments.len() {
        if arguments[index] == "with"
            && index + 2 < arguments.len()
            && arguments[index + 1] == "style"
        {
            style = arguments[index + 2].as_str();
            break;
        }
        index += 1;
    }

    match style {
        "brief" => {
            println!("v{}", env!("CARGO_PKG_VERSION"));
        }
        "normal" => {
            println!(
                "The nekonomicon's is now at version {}.",
                env!("CARGO_PKG_VERSION")
            );
        }
        "full" => {
            println!(
                "I will tell you a story about The Nekonomicon who is now {}.",
                env!("CARGO_PKG_VERSION")
            );
            println!();
            println!("Back in my old days...");
            println!("+ V0.1.0:");
            println!();
            println!("- Initial release with minimal parser");
            println!("- Support for 'say' command with string literals");
            println!("- CLI interface with flexible command system");
            println!("- Basic error handling and script execution");
            println!();
            println!("A magical scripting language for automation and clarity.");
        }
        _ => {
            eprintln!("Unknown style '{style}'. Supported styles: brief, normal, full");
            std::process::exit(1);
        }
    }
}
