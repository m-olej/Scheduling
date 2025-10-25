use scheduling::file_handler;
use scheduling::problem_1::models::{Instance, Schedule, Task};
use scheduling::problem_1::verify::{calculate_score, verify_instance};
use std::time::Instant;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = std::path::Path::new(&args[1]);
    let output_file = std::path::Path::new(&args[2]);

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

    // Readonly parse instance
    let instance = Instance::read(&file_content);

    // Mutable algo structures
    let mut schedule: Schedule = Schedule::new(instance.tasks.iter().collect());

    let start_time = Instant::now();
    // Start algo

    schedule.tasks.sort_by_key(|t| t.ready_time);

    // End algo
    let duration = start_time.elapsed();

    let score = calculate_score(&schedule.tasks);

    schedule.set_duration(duration.as_millis());
    schedule.set_score(score);

    file_handler::write_to_file(output_file, schedule.format());
}
