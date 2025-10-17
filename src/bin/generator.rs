use rand::prelude::*;
use scheduling::file_handler;
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

    let data = vec![b'a'; size];

    file_handler::write_to_file(&file_path, &data);
}
