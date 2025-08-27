use sap::{Argument, Parser};

fn main() {
    let mut parser = Parser::from_env().unwrap();

    while let Some(arg) = parser.forward().unwrap() {
        match arg {
            Argument::Short('h') | Argument::Long("help") => print_help(parser.name()),
            Argument::Short('v') | Argument::Long("version") => print_version(),
            _ => {}
        }
    }
}

fn print_help(program_name: &str) {
    println!("Usage: {}, <COMMAND> [OPTIONS] [ARGUMENTS]", program_name);
    println!(
        "A simple package manager for installing, updating, searching, querying and removing packages."
    );

    println!("\nCommands:");
    println!("  install <package>\t\tInstall a package");
    println!("  remove <package>\t\tRemove a package");
    println!("  update <package>\t\tUpdate a package");
    println!("  upgrade\t\t\tUpdate all packages");
    println!("  list [query]\t\t\tList all installed packages");
    println!("  search <query>\t\tSearch for a package");
    println!("  info <package>\t\tGet info about a package");

    println!("\nOptions:");
    println!("  -h, --help\t\t\tShow this help message");
    println!("  -v, --version\t\t\tShow the version");
}

fn print_version() {
    println!("Hexuro Package Manager v{}", env!("CARGO_PKG_VERSION"));
}
