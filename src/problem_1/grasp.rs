use crate::problem_1::models::{Schedule, Task};
use crate::problem_1::verify::calculate_score;
use rand::{rng, Rng};

const ALPHA: f32 = 0.1;

pub fn generate_initial_input(tasks: Vec<&Task>) -> Schedule {
    let mut rng = rng();
    let mut initial_schedule: Vec<&Task> = Vec::with_capacity(tasks.len());
    let mut unscheduled_tasks: Vec<u32> = tasks.iter().map(|task| task.id).collect();
    let mut temp_schedule: Vec<&Task> = Vec::with_capacity(tasks.len());

    println!(
        "Generating initial solution using GRASP with ALPHA = {}",
        ALPHA
    );

    while unscheduled_tasks.len() > 0 {
        let mut candidate_list = Vec::new(); // Potentially optimize by calculating candidate count
        println!("remaining: {}", unscheduled_tasks.len());

        // For each unscheduled task
        for task_id in unscheduled_tasks.iter() {
            // For all posible insertion positions
            for pos in 0..initial_schedule.len() + 1 {
                // Create temporary schedule with the task inserted at position pos
                temp_schedule.clear();
                temp_schedule.extend_from_slice(&initial_schedule[..pos]);
                temp_schedule.push(tasks[*task_id as usize]);
                temp_schedule.extend_from_slice(&initial_schedule[pos..]);

                // Calculate score of the temporary schedule
                let score = calculate_score(&temp_schedule);

                // Add to candidates list
                candidate_list.push((*task_id, pos, score));
            }
        }

        // Sort candidate list
        candidate_list.sort_by(|c1, c2| c1.2.cmp(&c2.2));

        // Create RCL
        let top_candidate_count = (ALPHA * (candidate_list.len() as f32)).floor() as usize;
        let rcl = &candidate_list[0..top_candidate_count];

        // Pick random top candidate
        let (picked_task_id, picked_pos, _) = rcl[rng.random_range(0..top_candidate_count - 1)];

        // Insert the candidate into output schedule (task_id at pos)
        initial_schedule.insert(picked_pos, &tasks[picked_task_id as usize]);

        // Remove picked task_id from unscheduled tasks
        unscheduled_tasks.retain(|&x| x != picked_task_id);
    }

    Schedule::new(initial_schedule)
}
