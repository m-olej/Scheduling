use crate::problem_1::models::{Instance, Task};
use crate::problem_1::verify::calculate_score;
use std::collections::HashMap;

pub fn generate_initial_input(tasks: Vec<Task>, switch_times: HashMap<u32, u32>) -> Instance {
    let mut initial_schedule: Vec<Task> = Vec::with_capacity(tasks.len());
    let mut unscheduled_tasks: Vec<Task> = tasks.clone();

    while unscheduled_tasks.len() > 0 {
        let mut min_switch_time: Option<(usize, u32)> = None;

        // For each unscheduled task
        for (index, task) in unscheduled_tasks.iter().enumerate() {
            // For all posible insertion positions
            for pos in 0..initial_schedule.len() {
                // Create temporary schedule with the task inserted at position pos
                let temp_schedule = {
                    let mut temp: Vec<&Task> = initial_schedule.iter().collect();
                    temp.insert(pos, task);
                    temp
                };

                // Calculate score of the temporary schedule
                let score = calculate_score(temp_schedule);
            }
        }
    }

    Instance {
        n: tasks.len(),
        tasks: initial_schedule,
    }
}
