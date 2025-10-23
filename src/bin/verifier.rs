use scheduling::file_handler;
use scheduling::utils::verify_instance;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = std::path::Path::new(&args[1]);

    let file_content = file_handler::read_from_file(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", file_path.display(), err);
        std::process::exit(1);
    });

    if let Err(err_msg) = verify_instance(&file_content) {
        eprintln!("Verification failed: {}", err_msg);
        std::process::exit(1);
    }

    println!("Verification successful: The instance is valid.");
}
