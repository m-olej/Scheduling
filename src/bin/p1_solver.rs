use log::{debug, error, info};
use scheduling::file_handler;
use scheduling::problem_1::algo::bnb::solve;
use scheduling::problem_1::algo::bounds::pick_strategy;
use scheduling::problem_1::algo::heuristics::find_initial_solution;
use scheduling::problem_1::models::{GlobalBest, Instance, Solution};
use scheduling::problem_1::verify::verify_instance;
use std::time::{Duration, Instant};

fn main() {
    env_logger::init();
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        error!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = std::path::Path::new(&args[1]);
    let output_file = std::path::Path::new(&args[2]);

    let file_content = file_handler::read_from_file(input_file).unwrap_or_else(|err| {
        error!(
            "Error reading input file '{}': {}",
            input_file.display(),
            err
        );
        std::process::exit(1);
    });

    // Verify instance
    verify_instance(&file_content).unwrap_or_else(|err| {
        error!("Input instance verification failed: {}", err);
        std::process::exit(1);
    });

    // Readonly parse instance
    let mut instance = Instance::read(&file_content);

    // Data structures
    let mut solution = Solution::new(Vec::with_capacity(instance.n));

    let time_limit = Duration::new(instance.n as u64 / 10, 0);
    debug!(
        "Solving instance with {} tasks and time limit of {} seconds",
        instance.n,
        time_limit.as_secs()
    );
    let start_time = Instant::now();

    // Start algo

    // 1. Analyze instance
    instance.analyze();

    // 2. Choose bounding strategy
    let strategy = pick_strategy(instance.metrics.sts);

    // 3. Find initial solution (Upper bound)
    find_initial_solution(&instance, &mut solution, &strategy);

    let global_best = GlobalBest::new(solution);

    // 4. Run solver loop until time limit
    solve(
        &instance,
        strategy,
        global_best.clone(),
        start_time,
        time_limit,
    );

    // 5. Return best found solution
    let mut final_best = global_best.into_inner();

    // End algo
    let duration = start_time.elapsed();

    final_best.set_duration(duration.as_millis());

    debug!(
        "Analyzed Instance Metrics in {} ms:\n {}",
        duration.as_millis(),
        instance.metrics
    );

    info!(
        "Generated Solution in {} ms\n{}",
        duration.as_millis(),
        final_best.format()
    );

    file_handler::write_to_file(output_file, final_best.format());
}
