use crate::problem_2::models::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct SearchNode {
    // IDs of jobs scheduled so far
    pub scheduled_jobs: Vec<usize>,
    // reference to JobResults scheduled so far
    pub scheduled_results: Vec<JobResult>,
    // Remaining unscheduled jobs (bitmask or boolean vec is faster)
    pub unscheduled_mask: Vec<bool>,
    // Current state of machines (when they become free)
    pub machine_finish_times: Vec<f64>,
    // The objective value incurred SO FAR
    pub current_tardy_work: f64,
    // Heuristic estimate of remaining cost (for sorting)
    pub estimated_total_cost: f64,
}

impl SearchNode {
    pub fn new(num_jobs: usize, num_machines: usize) -> Self {
        SearchNode {
            scheduled_jobs: Vec::new(),
            scheduled_results: Vec::new(),
            unscheduled_mask: vec![true; num_jobs],
            machine_finish_times: vec![0.0; num_machines],
            current_tardy_work: 0.0,
            estimated_total_cost: 0.0,
        }
    }

    // Deterministic comparison for sorting
    pub fn cmp(&self, other: &Self) -> Ordering {
        // Lower cost is better
        self.estimated_total_cost
            .total_cmp(&other.estimated_total_cost)
            .then_with(|| self.current_tardy_work.total_cmp(&other.current_tardy_work))
            .then_with(|| self.scheduled_jobs.cmp(&other.scheduled_jobs))
    }
}
// Funkcja pomocnicza: znajdź najlepszą maszynę dla zadania (Greedy)
#[inline]
pub fn find_best_machine_assignment(
    job: &Job,
    machine_times: &[f64],
    machines: &[Machine],
) -> (usize, f64) {
    let mut best_m_idx = 0;
    let mut best_finish = f64::MAX;

    for (m_idx, m_params) in machines.iter().enumerate() {
        let t_free = machine_times[m_idx];
        let start = (t_free as f64).max(job.r_j as f64);
        let finish_time = start + job.p_j as f64 * m_params.b_k;

        if finish_time < best_finish {
            best_finish = finish_time;
            best_m_idx = m_idx;
        }
    }
    (best_m_idx, best_finish)
}

pub fn run_pilot_simulation(parent_node: &SearchNode, jobs: &[Job], machines: &[Machine]) -> f64 {
    // Klonujemy stan maszyn, aby nie psuć węzła
    let mut temp_machine_times = parent_node.machine_finish_times.clone();

    let mut future_cost: f64 = 0.0;

    // Zbieramy wskaźniki do niezaplanowanych zadań
    let pending_jobs: Vec<&Job> = jobs
        .iter()
        .enumerate()
        .filter(|(idx, _)| parent_node.unscheduled_mask[*idx])
        .map(|(_, job)| job)
        .collect();

    for job in pending_jobs {
        let (best_m, finish_time) =
            find_best_machine_assignment(job, &temp_machine_times, machines);

        // Aktualizacja stanu symulacji
        temp_machine_times[best_m] = finish_time;

        // Obliczenie kosztu Y_j
        let tardiness = (finish_time - job.d_j as f64).max(0.0);
        let tardy_work =
            tardiness.min(job.p_j as f64 * machines[best_m].b_k) / machines[best_m].b_k;
        future_cost += tardy_work;
    }

    future_cost
}
