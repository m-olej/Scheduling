use crate::problem_1::algo::bounds::calculate_lb;
use crate::problem_1::algo::bounds::BoundingStrategy;
use crate::problem_1::models::{GlobalBest, Instance, Task};
use log::debug;
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Node<'a> {
    pub partial_sequence: Vec<&'a Task>, // Uporządkowane zadania
    pub remaining_jobs: HashSet<usize>,  // Zadania do uporządkowania
    pub current_time: u64,               // Czas zakończenia ostatniego zadania
    pub current_cost: u64,               // Sum C_j dla zadań w partial_sequence
    pub last_job_idx: Option<usize>,     // Indeks ostatniego zadania w partial_sequence
}

pub fn solve<'a>(
    instance: &'a Instance,
    strategy: BoundingStrategy,
    global_best: GlobalBest<'a>,
    start_time: Instant,
    time_limit: Duration,
) {
    debug!(
        "Starting B&B with strategy {:?} on instance with {} tasks",
        strategy, instance.n
    );
    // Stworzenie węzła początkowego (korzenia)
    let mut initial_remaining = HashSet::with_capacity(instance.n);
    for i in 0..instance.n {
        initial_remaining.insert(i);
    }

    let root_node = Node {
        partial_sequence: Vec::with_capacity(instance.n),
        remaining_jobs: initial_remaining,
        current_time: 0,
        current_cost: 0,
        last_job_idx: None,
    };

    // Uruchomienie rekurencyjnego, wielowątkowego B&B
    bnb_recursive(
        instance,
        strategy,
        global_best,
        root_node,
        start_time,
        time_limit,
    );
}

/// Rekurencyjna funkcja B&B.
fn bnb_recursive<'a>(
    instance: &'a Instance,
    strategy: BoundingStrategy,
    global_best: GlobalBest<'a>,
    node: Node<'a>,
    start_time: Instant,
    time_limit: Duration,
) {
    debug!("Current node: {:?}", node);
    // --- 1. Sprawdzenie Limitu Czasu ---
    // Kluczowe dla spełnienia wymagań n/10 sekundy.
    if start_time.elapsed() > time_limit {
        return;
    }

    // --- 2. Sprawdzenie Warunku Podstawowego (Liść Drzewa) ---
    if node.remaining_jobs.is_empty() {
        // Jesteśmy w liściu, mamy pełną sekwencję. Koszt jest już obliczony.
        let solution_cost = node.current_cost;
        let current_best_cost = global_best.score.load(Ordering::Relaxed);

        if solution_cost < current_best_cost {
            // Znaleźliśmy lepsze rozwiązanie.
            // Zablokuj mutex i zaktualizuj globalny stan.
            let mut solution_guard = global_best.solution.lock().unwrap();

            // Sprawdź ponownie, na wypadek gdyby inny wątek nas ubiegł
            if solution_cost < solution_guard.get_score() {
                debug!(
                    "Nowe UB: {} (Poprzednie: {})",
                    solution_cost,
                    solution_guard.get_score()
                );
                solution_guard.set_score(solution_cost);
                solution_guard.tasks = node.partial_sequence;

                // Zaktualizuj atomową wartość (szybki odczyt dla innych wątków)
                global_best.score.store(solution_cost, Ordering::Relaxed);
            }
        }
        return; // Zakończ tę gałąź
    }

    debug!(
        "Exploring node with {} remaining jobs, current cost: {}",
        node.remaining_jobs.len(),
        node.current_cost
    );
    // --- 3. Rozgałęzienie (Branching) ---
    let last_job_idx = node
        .partial_sequence
        .last()
        .cloned()
        .map(|task| task.id as usize);

    // Stwórz listę potencjalnych dzieci do eksploracji
    let mut children_to_explore = Vec::new();

    for &next_job_idx in &node.remaining_jobs {
        debug!("Generating child node by adding job {}", next_job_idx);
        // Oblicz "prawdziwy" koszt dodania tego jednego zadania
        let p_j = instance.tasks[next_job_idx].processing_time as u64;
        let r_j = instance.tasks[next_job_idx].ready_time as u64;
        let s_ij = if let Some(last_idx) = last_job_idx {
            instance.tasks[last_idx].switch_time[next_job_idx] as u64
        } else {
            0 // S_0j
        };

        let start_time = (node.current_time + s_ij).max(r_j);
        let completion_time = start_time + p_j;

        let mut child_node = node.clone(); // Klonujemy stan rodzica
        child_node
            .partial_sequence
            .push(&instance.tasks[next_job_idx]);
        child_node.remaining_jobs.remove(&next_job_idx);
        child_node.current_time = completion_time;
        child_node.current_cost = node.current_cost + completion_time;
        child_node.last_job_idx = Some(next_job_idx);

        // --- 4. Obliczenie Granicy (Bounding) ---
        // Oblicz LB *przed* rekurencyjnym wywołaniem
        let lower_bound = calculate_lb(&child_node, &instance, &strategy);

        debug!(
            "Child node with next job {} has LB: {}",
            next_job_idx, lower_bound
        );

        // --- 5. Przycinanie (Pruning) ---
        if lower_bound < global_best.score.load(Ordering::Relaxed) {
            children_to_explore.push((child_node, lower_bound));
            debug!(
                "Child node with next job {} added to exploration",
                next_job_idx
            );
        } else {
            debug!(
                "Child node with next job {} pruned (LB: {}, UB: {})",
                next_job_idx,
                lower_bound,
                global_best.score.load(Ordering::Relaxed)
            );
        }
    }

    // Sortuj dzieci wg dolnej granicy (Best-First search) - opcjonalne, ale zwykle pomaga
    children_to_explore.sort_by_key(|(_, lb)| *lb);

    debug!(
        "Exploring {} child nodes from current node",
        children_to_explore.len()
    );

    // --- 6. Wielowątkowa Eksploracja (Parallelism) ---
    // Użyj Rayon do równoległego przetworzenia wszystkich obiecujących dzieci.
    // Rayon automatycznie zarządza pulą wątków i kradzieżą pracy (work-stealing).
    children_to_explore.into_par_iter().for_each(|(child, _)| {
        // _ to lb, nie jest już potrzebne
        bnb_recursive(
            instance,
            strategy,
            global_best.clone(),
            child,
            start_time,
            time_limit,
        );
    });
}
