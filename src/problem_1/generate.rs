use crate::problem_1::models::{Instance, Task};
use rand::rngs::ThreadRng;
use rand::{rng, Rng};

const MIN_P: u32 = 1;
const MAX_P: u32 = 35;
const MIN_S: u32 = 1;
const MAX_S: u32 = 20;
const SPREAD: u32 = 2; // [1, 3]

pub fn generate_instance(n: usize) -> Instance {
    let mut rng: ThreadRng = rng();

    let processing_times: Vec<u32> = (0..n).map(|_| rng.random_range(MIN_P..MAX_P)).collect();

    let mut setup_times: Vec<Vec<u32>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i != j {
                setup_times[i][j] = rng.random_range(MIN_S..MAX_S);
            }
        }
    }

    let total_processing_time: u32 = processing_times.iter().sum();
    let ready_time_upper_bound: u32 = total_processing_time / SPREAD;

    let ready_times: Vec<u32> = processing_times
        .iter()
        .map(|p_j| {
            let inverse_factor = (MAX_P - p_j) as f32 / MAX_P as f32;

            let min_ready_time = (inverse_factor * ready_time_upper_bound as f32) as u32;

            if min_ready_time >= ready_time_upper_bound {
                return ready_time_upper_bound;
            }

            rng.random_range(min_ready_time..ready_time_upper_bound)
        })
        .collect();

    let tasks: Vec<Task> = (0..n)
        .map(|i| Task {
            id: i as u32,
            ready_time: ready_times[i],
            processing_time: processing_times[i],
            switch_time: setup_times[i].clone(),
        })
        .collect();

    Instance {
        n: tasks.len(),
        tasks: tasks,
    }
}
