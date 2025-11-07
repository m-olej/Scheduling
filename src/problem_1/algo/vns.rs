use super::neighborhood::{Neighborhood, Relocate, Swap};
use super::schedule::Schedule;
use crate::problem_1::models::{Solution, Task};
use std::time::{Duration, Instant};

pub struct VnsSolver<'a> {
    pub neighborhoods: Vec<Box<dyn Neighborhood>>,
    pub tasks_ref: &'a Vec<Task>,
    pub time_limit: Duration,
}

impl<'a> VnsSolver<'a> {
    pub fn new(tasks_ref: &'a Vec<Task>, time_limit: Duration) -> Self {
        // in order of increasing complexity
        let neighborhoods: Vec<Box<dyn Neighborhood>> = vec![
            Box::new(Swap {}),
            Box::new(Relocate {}),
            // implement 2-opt if needed
        ];
        Self {
            neighborhoods,
            tasks_ref,
            time_limit,
        }
    }

    pub fn variable_neighborhood_descent(&self, schedule: &mut Schedule, run_time: Instant) {
        let mut k = 0;
        while k < self.neighborhoods.len() {
            if run_time.elapsed() >= self.time_limit {
                break;
            }
            // println!(" VND iteration with neighborhood {}", k);

            let improved = self.neighborhoods[k].find_best_move(schedule);
            if improved {
                // println!("  Improvement found in neighborhood {}", k);
                k = 0; // restart from the first neighborhood
            } else {
                // println!("  No improvement found in neighborhood {}", k);
                k += 1; // move to the next neighborhood
            }
        }
    }

    pub fn solve(&self, initial_task_ids: Vec<u32>, run_time: Instant) -> Solution {
        let mut schedule = Schedule::new(initial_task_ids, self.tasks_ref);

        println!("Initial solution score: {}", schedule.score);

        let k_max = self.neighborhoods.len();

        while run_time.elapsed() < self.time_limit {
            let mut k = 0;
            while k < k_max {
                if run_time.elapsed() >= self.time_limit {
                    break;
                }
                // println!("VNS iteration with neighborhood {}", k);
                // 1. Shaking
                let mut working_schedule = schedule.clone();
                self.neighborhoods[k].shake(&mut working_schedule, k);
                working_schedule.calculate_full_score();
                // println!("Shaken solution score: {}", working_schedule.score);
                // 2. Intensification
                self.variable_neighborhood_descent(&mut working_schedule, run_time);
                // println!("After VND solution score: {}", working_schedule.score);

                // 3. Move or not
                if working_schedule.score < schedule.score {
                    schedule = working_schedule;
                    // println!("New best solution score: {}", schedule.score);
                    k = 0; // restart from the first neighborhood
                } else {
                    k += 1; // move to the next neighborhood
                            // println!("No improvement, moving to neighborhood {}", k);
                }
            }
        }

        println!("Final solution score: {}", schedule.score);

        let tasks = schedule
            .task_ids
            .iter()
            .map(|&id| &self.tasks_ref[id as usize])
            .collect();

        Solution::new(tasks)
    }
}
