use std::vec;

use crate::problem_1::models::{Schedule, Task};
use rand::{rng, Rng};

const ALPHA: f32 = 0.3;

fn calculate_append_score(
    curr_schedule: &Vec<&Task>,
    last_score: u32,
    task_to_insert: &Task,
) -> u32 {
    if curr_schedule.len() == 0 {
        return task_to_insert.ready_time + task_to_insert.processing_time;
    }

    let last_task = curr_schedule[curr_schedule.len() - 1];
    let recalibration_time = last_task.switch_time[task_to_insert.id as usize];

    let score = task_to_insert
        .ready_time
        .max(last_score + recalibration_time)
        + task_to_insert.processing_time;

    score
}

pub fn generate_initial_input(tasks: Vec<&Task>) -> Schedule {
    let mut rng = rng();

    let mut initial_schedule: Vec<&Task> = Vec::with_capacity(tasks.len());
    let mut scheduled_tasks: Vec<bool> = vec![false; tasks.len()];

    let mut last_score = 0;

    println!(
        "Generating initial solution using GRASP with ALPHA = {}",
        ALPHA
    );

    while scheduled_tasks.iter().any(|&x| !x) {
        let mut candidate_list = Vec::new();

        // For each unscheduled task
        for (task_id, task) in tasks.iter().enumerate() {
            if scheduled_tasks[task_id] {
                continue; // Skip already scheduled tasks
            }

            // Calculate score if inserted at the end
            let score = calculate_append_score(&initial_schedule, last_score, *task);

            // Add to candidates list
            candidate_list.push((task_id, score));
        }

        // Sort candidate list
        candidate_list.sort_unstable_by_key(|k| k.1);

        // Create RCL
        let top_candidate_count = (ALPHA * (candidate_list.len() as f32)).floor() as usize;
        let rcl = &candidate_list[0..top_candidate_count + 1];

        // Pick random top candidate
        let (picked_task_id, picked_score) = rcl[rng.random_range(0..top_candidate_count + 1)];
        last_score = picked_score;

        // Insert the candidate into output schedule (task_id at pos)
        initial_schedule.push(&tasks[picked_task_id]);

        // Remove picked task_id from unscheduled tasks
        scheduled_tasks[picked_task_id] = true;
    }

    Schedule::new(initial_schedule)
}
