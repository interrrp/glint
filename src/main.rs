use clap::Parser;

use glint::check_all;

mod args;

fn main() {
    let args = args::Args::parse();

    for path in args.paths {
        println!("-- {}", path.display());

        match std::fs::read_to_string(&path) {
            Ok(contents) => {
                let results = check_all(&contents);
                for result in results {
                    println!("{}", result);
                }
            }
            Err(error) => {
                println!("Could not read from {}: {}", path.display(), error);
            }
        }

        println!();
    }
}
