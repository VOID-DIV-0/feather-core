///
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
            println!("Ah! The nekonomicon is at version {}.", env!("CARGO_PKG_VERSION"));
        }
        "full" => {
            println!("Nekonomicon Interpreter");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!();
            println!("Release Notes:");
            println!("- Initial release with minimal parser");
            println!("- Support for 'say' command with string literals");
            println!("- CLI interface with flexible command system");
            println!("- Basic error handling and script execution");
            println!();
            println!("A magical scripting language for automation and clarity.");
        }
        _ => {
            eprintln!("Unknown style '{}'. Supported styles: brief, normal, full", style);
            std::process::exit(1);
        }
    }
}