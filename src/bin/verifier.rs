use scheduling::file_handler;
use scheduling::problem_1::models::Instance;
use scheduling::problem_1::verify::{verify_instance, verify_solution};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Pass instance file or solution file and instance file to verify either instance or solution.");
        eprintln!("Usage: {} <input_file> [instance_file]", args[0]);
        std::process::exit(1);
    }

    if args.len() == 3 {
        let solution_file_path = std::path::Path::new(&args[1]);
        let instance_file_path = std::path::Path::new(&args[2]);

        let solution_content =
            file_handler::read_from_file(solution_file_path).unwrap_or_else(|err| {
                eprintln!(
                    "Error reading solution file '{}': {}",
                    solution_file_path.display(),
                    err
                );
                std::process::exit(1);
            });

        let instance_content =
            file_handler::read_from_file(instance_file_path).unwrap_or_else(|err| {
                eprintln!(
                    "Error reading instance file '{}': {}",
                    instance_file_path.display(),
                    err
                );
                std::process::exit(1);
            });

        let instance = Instance::read(&instance_content);

        if let Err(err_msg) = verify_solution(&solution_content, &instance) {
            eprintln!("Solution verification failed: {}", err_msg);
            std::process::exit(1);
        }

        println!("Solution verification successful: The solution is valid.");
    } else {
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
}
