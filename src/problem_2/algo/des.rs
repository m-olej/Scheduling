use crate::problem_2::models::*;
use std::collections::BinaryHeap;

pub struct ScheduleResult {
    pub rule_name: String,
    pub schedule: Vec<JobResult>,
    pub total_tardy_work: i64,
}

pub fn run_simulation(
    jobs: &Vec<Job>,
    machines: &Vec<Machine>,
    priority_rule: &dyn PriorityRule,
) -> ScheduleResult {
    let num_jobs = jobs.len();
    let mut machine_heap: BinaryHeap<MachineState> = BinaryHeap::new();
    for machine in machines.iter() {
        machine_heap.push(MachineState {
            machine_id: machine.id,
            free_time: 0,
        });
    }

    let mut is_scheduled: Vec<bool> = vec![false; num_jobs];
    let mut schedule_results: Vec<JobResult> = Vec::with_capacity(num_jobs);
    let mut total_tardy_work: i64 = 0;

    for _ in 0..num_jobs {
        let mut earliest_machine = machine_heap.pop().unwrap();
        let t_free = earliest_machine.free_time;

        // Krok (c-e): Znajdź najlepsze nieplanowane zadanie
        let (best_job_idx, _) = jobs
            .iter()
            .enumerate()
            .filter(|(idx, _)| !is_scheduled[*idx])
            .min_by_key(|(_, job)| priority_rule.calculate(t_free, job))
            .expect("No unscheduled jobs available");

        is_scheduled[best_job_idx] = true;
        let best_job = &jobs[best_job_idx];
        let machine_params = &machines[earliest_machine.machine_id];

        // Krok (f): Oblicz czasy
        let t_start = t_free.max(best_job.r_j);
        // Użyj arytmetyki stałoprzecinkowej
        let t_proc = best_job.p_j * machine_params.b_k / STANDARDIZATION_FACTOR;
        let t_complete = t_start + t_proc;

        // Krok (g-i): Zaktualizuj stan
        earliest_machine.free_time = t_complete;
        machine_heap.push(earliest_machine);

        // Oblicz Y_j
        let tardiness = (t_complete - best_job.d_j).max(0);
        let tardy_work = tardiness.min(best_job.p_j * machine_params.b_k / STANDARDIZATION_FACTOR)
            / machine_params.b_k
            * STANDARDIZATION_FACTOR;
        total_tardy_work += tardy_work;

        schedule_results.push(JobResult {
            job_id: best_job.id,
            machine_id: machine_params.id,
            completion_time: t_complete,
            tardy_work: tardy_work,
        });
    }

    ScheduleResult {
        rule_name: priority_rule.name().to_string(),
        total_tardy_work,
        schedule: schedule_results,
    }
}
