use crate::problem_1::models::Task;

#[derive(Debug, Clone)]
pub struct Schedule<'a> {
    pub task_ids: Vec<u32>,
    pub score: u64,
    pub score_partials: Vec<u64>,
    pub time_partials: Vec<u64>,

    tasks_ref: &'a Vec<Task>,
}

impl<'a> Schedule<'a> {
    pub fn new(initial_task_ids: Vec<u32>, tasks_ref: &'a Vec<Task>) -> Self {
        let mut schedule = Self {
            task_ids: initial_task_ids,
            score: 0, // to be calculated later
            score_partials: vec![0; tasks_ref.len()],
            time_partials: vec![0; tasks_ref.len()],
            tasks_ref,
        };

        schedule.calculate_full_score();

        schedule
    }

    #[inline(always)]
    pub fn recalculate_score_from_index(&mut self, start_idx: usize) -> u64 {
        let n = self.task_ids.len();
        if start_idx >= n {
            return self.score;
        }

        // start_idx is the idx of the latest element to be unchanged not the task_id
        let mut total_score = if start_idx == 0 {
            0
        } else {
            self.score_partials[start_idx - 1]
        };

        let mut current_time: u64 = if start_idx == 0 {
            0
        } else {
            self.time_partials[start_idx - 1]
        };

        for (i, &task_id) in self.task_ids.iter().enumerate().skip(start_idx) {
            let task = &self.tasks_ref[task_id as usize];
            let setup_time = if i == 0 {
                0
            } else {
                let prev_task_id = self.task_ids[i - 1];
                self.tasks_ref[prev_task_id as usize].switch_time[task_id as usize]
            };

            let start_time = (current_time + setup_time as u64).max(task.ready_time as u64);
            current_time = start_time + task.processing_time as u64;
            total_score += current_time;
        }

        total_score
    }

    /// Calculates the full score of the current task sequence from scratch.
    /// Creates the score_partials array as a side effect.
    /// This should only be used when the schedule is created or after a large change (like in `shake`).
    pub fn calculate_full_score(&mut self) {
        if self.task_ids.is_empty() {
            self.score = 0;
            self.score_partials.clear();
        }

        let mut total_completion_time: u64 = 0;
        let mut current_time: u64 = 0;

        // OPTIMIZATION: Using an iterator chain is idiomatic Rust and allows the compiler
        // to perform optimizations like loop fusion and bounds-check elimination.
        for (i, &task_id) in self.task_ids.iter().enumerate() {
            let task = &self.tasks_ref[task_id as usize];
            let setup_time = if i == 0 {
                0
            } else {
                let prev_task_id = self.task_ids[i - 1];
                self.tasks_ref[prev_task_id as usize].switch_time[task_id as usize]
            };

            // Using u64 for all time calculations prevents integer overflow.
            let start_time = (current_time + setup_time as u64).max(task.ready_time as u64);
            current_time = start_time + task.processing_time as u64;
            total_completion_time += current_time;
            self.score_partials[i] = total_completion_time;
            self.time_partials[i] = current_time;
        }
        self.score = total_completion_time;
    }
}
