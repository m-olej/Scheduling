use super::schedule::Schedule;
use rand::{rng, Rng};

pub trait Neighborhood {
    // Applies the best move found in the neighborhood to the given schedule.
    /// Returns true if a better neighbor was found and applied, false otherwise.
    fn find_best_move(&self, schedule: &mut Schedule) -> bool;

    // Applies random move from this neighborhood to the schedule.
    fn shake(&self, schedule: &mut Schedule, k: usize);
}

pub struct Swap;
impl Neighborhood for Swap {
    fn find_best_move(&self, schedule: &mut Schedule) -> bool {
        let n = schedule.task_ids.len();
        if n < 2 {
            return false;
        }

        let original_score = schedule.score;

        for i in 0..n {
            for j in i + 1..n {
                schedule.task_ids.swap(i, j);
                let new_score = schedule.recalculate_score_from_index(i);
                // println!(
                //     "   Considering swap of positions {} and {}, new score: {}",
                //     i, j, new_score
                // );
                if new_score < original_score {
                    // println!(
                    //     "   Swap of positions {} and {} improves score from {} to {}",
                    //     i, j, original_score, new_score
                    // );
                    schedule.score = new_score;
                    return true;
                }
                schedule.task_ids.swap(i, j); // revert
            }
        }

        false
    }

    fn shake(&self, schedule: &mut Schedule, k: usize) {
        let mut rng = rng();
        let n = schedule.task_ids.len();
        if n < 2 {
            return;
        }

        println!("   Shaking with {} random swaps", k);

        for _ in 0..k {
            let i = rng.random_range(0..n);
            let mut j = rng.random_range(0..n);
            while j == i {
                j = rng.random_range(0..n);
            }
            schedule.task_ids.swap(i, j);
        }
    }
}

pub struct Relocate;
impl Neighborhood for Relocate {
    fn find_best_move(&self, schedule: &mut Schedule) -> bool {
        let n = schedule.task_ids.len();
        if n < 2 {
            return false;
        }

        let current_score = schedule.score;

        for task_idx in 0..n {
            let task_id = schedule.task_ids.remove(task_idx);

            for insert_pos in 0..n {
                if insert_pos == task_idx {
                    continue;
                }
                schedule.task_ids.insert(insert_pos, task_id);
                let new_score =
                    schedule.recalculate_score_from_index(usize::min(task_idx, insert_pos));
                // println!(
                // "   Considering relocate of task {} from pos {} to {}, new score: {}",
                // task_id, task_idx, insert_pos, new_score
                // );
                if new_score < current_score {
                    // println!(
                    //     "   Relocate of task {} from pos {} to {} improves score from {} to {}",
                    //     task_id, task_idx, insert_pos, current_score, new_score
                    // );
                    schedule.score = new_score;
                    return true;
                }
                schedule.task_ids.remove(insert_pos);
            }
            schedule.task_ids.insert(task_idx, task_id); // revert
        }

        false
    }

    fn shake(&self, schedule: &mut Schedule, k: usize) {
        let mut rng = rng();
        let n = schedule.task_ids.len();
        if n < 2 {
            return;
        }

        println!("   Shaking with {} random relocates", k);
        for _ in 0..k {
            let from = rng.random_range(0..n);
            let mut to = rng.random_range(0..n);
            while to == from {
                to = rng.random_range(0..n);
            }
            let task_id = schedule.task_ids.remove(from);
            schedule.task_ids.insert(to, task_id);
        }
    }
}
