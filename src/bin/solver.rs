use scheduling::file_handler;
use scheduling::utils::verify_instance;
use scheduling::Instance;

fn main() {
    println!("Implement solver here");
    // argv:
    // - input file
    // - output file

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = std::path::Path::new(&args[1]);
    // let output_file = std::path::Path::new(&args[2]);

    // Implement solver logic here
    let file_content = file_handler::read_from_file(input_file).unwrap_or_else(|err| {
        eprintln!(
            "Error reading input file '{}': {}",
            input_file.display(),
            err
        );
        std::process::exit(1);
    });

    // Verify instance
    verify_instance(&file_content).unwrap_or_else(|err| {
        eprintln!("Input instance verification failed: {}", err);
        std::process::exit(1);
    });

    let instance = Instance::read(&file_content);
    println!("Read instance: {:?}", instance);
}
