use scheduling::file_handler;
use scheduling::problem_1::meta_algo::grasp::generate_initial_input;
use scheduling::problem_1::meta_algo::vns::VnsSolver;
use scheduling::problem_1::models::Instance;
use scheduling::problem_1::verify::{calculate_score, verify_instance};
use std::time::{Duration, Instant};

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

    let time_limit = Duration::new(instance.n as u64 / 10, 0);
    println!(
        "Solving instance with {} tasks and time limit of {} seconds",
        instance.n,
        time_limit.as_secs()
    );
    let start_time = Instant::now();
    // Start algo

    // Mutable algo structures
    let initial_solution = generate_initial_input(instance.tasks.iter().collect());
    let vns_solver = VnsSolver::new(&instance.tasks, time_limit);
    let mut solution = vns_solver.solve(initial_solution, start_time);

    // End algo
    let duration = start_time.elapsed();

    let score = calculate_score(&solution.tasks);

    solution.set_duration(duration.as_millis());
    solution.set_score(score);

    println!(
        "Generated Solution in {} ms\n{}",
        duration.as_millis(),
        solution.format()
    );

    file_handler::write_to_file(output_file, solution.format());
}
