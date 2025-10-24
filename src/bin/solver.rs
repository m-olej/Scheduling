use scheduling::file_handler;
use scheduling::utils::{calculate_score, verify_instance};
use scheduling::{Instance, Schedule};

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
    let output_file = std::path::Path::new(&args[2]);

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

    let mut instance = Instance::read(&file_content);

    instance
        .tasks
        .sort_by(|a, b| a.ready_time.cmp(&b.ready_time));

    let mut task_times: Vec<(u32, u32)> = Vec::with_capacity(instance.n);
    for i in 0..instance.n {
        task_times.push((instance.tasks[i].ready_time, i as u32));
    }

    let schedule = Schedule {
        duration: 0,
        score: 0,
        tasks: task_times,
    };

    let output = calculate_score(schedule, instance);

    file_handler::write_to_file(output_file, output);
}
