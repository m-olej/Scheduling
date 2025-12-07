use crate::problem_2::algo::beam_search::{
    find_best_machine_assignment, run_pilot_simulation, SearchNode,
};
use crate::problem_2::models::*;
use rayon::prelude::*;
use std::collections::BinaryHeap;

pub struct ScheduleResult {
    pub rule_name: String,
    pub schedule: Vec<JobResult>,
    pub total_tardy_work: f64,
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
            free_time: 0.0,
        });
    }

    let mut is_scheduled: Vec<bool> = vec![false; num_jobs];
    let mut schedule_results: Vec<JobResult> = Vec::with_capacity(num_jobs);
    let mut total_tardy_work: f64 = 0.0;

    for _ in 0..num_jobs {
        let mut earliest_machine = machine_heap.pop().unwrap();
        let t_free = earliest_machine.free_time;

        // Krok (c-e): Znajdź najlepsze nieplanowane zadanie
        let (best_job_idx, _) = jobs
            .par_iter()
            .enumerate()
            .filter(|(idx, _)| !is_scheduled[*idx])
            .min_by(|(_, job), (_, other_job)| {
                priority_rule
                    .calculate(t_free, job, &machines[earliest_machine.machine_id])
                    .total_cmp(&priority_rule.calculate(
                        t_free,
                        other_job,
                        &machines[earliest_machine.machine_id],
                    ))
            })
            .expect("No unscheduled jobs available");

        is_scheduled[best_job_idx] = true;
        let best_job = &jobs[best_job_idx];
        let machine_params = &machines[earliest_machine.machine_id];

        // Krok (f): Oblicz czasy
        let t_start = t_free.max(best_job.r_j as f64);
        // Użyj arytmetyki stałoprzecinkowej
        let t_proc = best_job.p_j as f64 * machine_params.b_k;
        let t_complete = t_start + t_proc;

        // Krok (g-i): Zaktualizuj stan
        earliest_machine.free_time = t_complete;
        machine_heap.push(earliest_machine);

        // Oblicz Y_j
        let tardiness = (t_complete - best_job.d_j as f64).max(0.0);
        let tardy_work =
            tardiness.min(best_job.p_j as f64 * machine_params.b_k) / machine_params.b_k;
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

pub fn run_beam_search(
    jobs: &Vec<Job>,
    machines: &Vec<Machine>,
    priority_rule: &dyn PriorityRule,
) -> ScheduleResult {
    // Zwraca (koszt, kolejność zadań)

    let n = jobs.len();
    let m = machines.len();

    let beam_width = n;
    let branch_limit = 20;

    // 1. Inicjalizacja
    let root = SearchNode::new(n, m);
    let mut beam = vec![root];

    // Dane do Rayona (wrapowanie w Arc nie jest konieczne dla referencji, ale ułatwia borrow checker)
    let jobs_ref = jobs.as_slice();
    let machines_ref = machines.as_slice();

    // 2. Pętla po kolejnych poziomach drzewa (dodajemy jedno zadanie na poziom)
    for _level in 0..n {
        // A. Generowanie kandydatów (Równolegle)
        // flat_map generuje listę wszystkich dzieci ze wszystkich węzłów w beamie
        let mut next_candidates: Vec<SearchNode> = beam
            .par_iter()
            .flat_map(|parent| {
                // Znajdź 'branch_limit' najlepszych kandydatów na następny ruch dla tego rodzica.
                // Zamiast sprawdzać wszystkie N zadań, sprawdzamy te z najmniejszym MDD względem obecnego czasu.
                // To drastycznie przyspiesza algorytm (Heuristic Filter).

                let mut candidates_indices: Vec<usize> = parent
                    .unscheduled_mask
                    .iter()
                    .enumerate()
                    .filter(|(_, &active)| active)
                    .map(|(idx, _)| idx)
                    .collect();

                candidates_indices.sort_by(|&id_a, &id_b| {
                    let j_a = &jobs_ref[id_a];
                    let j_b = &jobs_ref[id_b];
                    let duration_a = (j_a.p_j as f64 * machines_ref[0].b_k).max(0.1); // unikanie dzielenia przez zero
                    let duration_b = (j_b.p_j as f64 * machines_ref[0].b_k).max(0.1);
                    let mdd_a =
                        (j_a.d_j as f64 - duration_a) - parent.machine_finish_times[0] as f64;
                    let mdd_b =
                        (j_b.d_j as f64 - duration_b) - parent.machine_finish_times[0] as f64;
                    mdd_a.total_cmp(&mdd_b)
                });

                // Bierzemy tylko TOP K
                let top_k_indices = candidates_indices.into_iter().take(branch_limit);

                // Dla każdego z Top K stwórz nowy węzeł
                top_k_indices
                    .map(|job_idx| {
                        let mut child = parent.clone();
                        let job = &jobs_ref[job_idx];

                        // 1. Przypisz zadanie do najlepszej maszyny (Deterministyczne Greedy)
                        let (best_m, finish_time) = find_best_machine_assignment(
                            job,
                            &child.machine_finish_times,
                            machines_ref,
                        );

                        // 2. Aktualizuj stan dziecka
                        child.machine_finish_times[best_m] = finish_time;
                        child.unscheduled_mask[job_idx] = false;
                        child.scheduled_jobs.push(job.id); // Zakładamy, że Job ma pole id

                        // 3. Oblicz koszt rzeczywisty tego kroku

                        let tardiness = (finish_time - job.d_j as f64).max(0.0);
                        let raw_work = tardiness.min(job.p_j as f64 * machines[best_m].b_k);
                        let tardy_work = (raw_work) / machines[best_m].b_k;
                        child.scheduled_results.push(JobResult {
                            job_id: job.id,
                            machine_id: best_m,
                            completion_time: finish_time,
                            tardy_work: tardy_work,
                        });
                        child.current_tardy_work += tardy_work;

                        // 4. PILOT: Oszacuj resztę (kosztowna operacja)
                        let future_estimate = run_pilot_simulation(&child, jobs_ref, machines_ref);
                        child.estimated_total_cost = child.current_tardy_work + future_estimate;

                        child
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // B. Selekcja (Pruning) - Sekwencyjnie dla determinizmu
        // Sortujemy wszystkich kandydatów
        next_candidates.sort_by(|a, b| a.cmp(b));

        // Zatrzymujemy tylko 'beam_width' najlepszych
        if next_candidates.len() > beam_width {
            next_candidates.truncate(beam_width);
        }

        beam = next_candidates;
    }

    // 3. Zwracamy najlepszy wynik z ostatniej warstwy
    let best_node = &beam[0];
    ScheduleResult {
        rule_name: priority_rule.name().to_string(),
        schedule: best_node.scheduled_results.clone(),
        total_tardy_work: best_node.current_tardy_work,
    }
}
