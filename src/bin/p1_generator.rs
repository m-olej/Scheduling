use scheduling::file_handler;
use scheduling::problem_1::generate::generate_instance as generate_problem_1_instance;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <size> <output_dir>", args[0]);
        std::process::exit(1);
    }

    let size: usize = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid size: {}", args[1]);
        std::process::exit(1);
    });

    let output_dir = &args[2];

    println!(
        "Generating file of size {} bytes in directory '{}'",
        size, output_dir
    );

    let file_path = Path::new(output_dir).join(format!("in_155927_{}.txt", size));

    // generate instance
    let instance = generate_problem_1_instance(size);

    // encode instance
    let data = instance.format();

    file_handler::write_to_file(&file_path, data);
}
