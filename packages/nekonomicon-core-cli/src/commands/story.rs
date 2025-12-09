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
/// let _ = story_handler();
/// ```
pub fn story_handler(args: &[String]) {
    // Default format
    let mut style = "normal";

    // Parse for "with style <value>"
    let mut i = 0;
    while i < args.len() {
        if args[i] == "with" && i + 2 < args.len() && args[i + 1] == "style" {
            style = args[i + 2].as_str();
            break;
        }
        i += 1;
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
