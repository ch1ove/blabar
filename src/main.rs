mod core;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if core::help::is_first_time() {
        // Display first-time help message
        core::help::display_help(true);
    } else if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        // Display regular help message
        core::help::display_help(false);
    } else {
        println!("Run 'blabar -h' for help.");
    }
}
