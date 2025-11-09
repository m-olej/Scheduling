use crate::problem_1::algo::bnb::Node;
use crate::problem_1::models::{Instance, Task};
use log::debug;

const STRATEGY_THRESHOLD: f64 = 0.5;

#[derive(Debug, Clone, Copy)]
pub enum BoundingStrategy {
    ATCS,   // Apparent Tardiness Cost with Setups based strategy
    INSERT, // Insertion based strategy
    HYBRID,
}

pub fn pick_strategy(sts: f64) -> BoundingStrategy {
    if sts < STRATEGY_THRESHOLD {
        BoundingStrategy::ATCS
    } else {
        BoundingStrategy::INSERT
    }
}

pub fn calculate_lb(node: &Node, instance: &Instance, strategy: &BoundingStrategy) -> u64 {
    debug!(
        "Calculating LB for node with {} remaining jobs using strategy {:?}",
        node.remaining_jobs.len(),
        strategy
    );
    // Dolna granica dla węzła to koszt, który już ponieśliśmy...
    let base_cost = node.current_cost;

    //...plus optymistyczne oszacowanie kosztu pozostałych zadań.
    let remaining_lb = match strategy {
        // POPRAWKA: Strategia ATCS (niski sts) powinna używać relaksacji SRPT,
        // ponieważ ignoruje ona S_ij, które są nieistotne. [1, 3]
        BoundingStrategy::ATCS => lb_srpt_relaxation(node, instance),
        // POPRAWKA: Strategia INSERT (wysoki sts) powinna używać relaksacji AP,
        // ponieważ skupia się ona na S_ij. [5, 3]
        BoundingStrategy::INSERT => lb_assignment_problem(node, instance),
        BoundingStrategy::HYBRID => {
            lb_assignment_problem(node, instance).max(lb_srpt_relaxation(node, instance))
        }
    };

    base_cost + remaining_lb
}

/// Implementuje relaksację Problemu Przypisania (AP) połączoną z logiką SPT.
/// Oblicza LB dla pozostałych zadań, gdy S_ij są znaczące.
fn lb_assignment_problem(node: &Node, instance: &Instance) -> u64 {
    debug!(
        "Calculating LB using Assignment Problem relaxation for {} remaining jobs",
        node.remaining_jobs.len()
    );
    let n_remaining = node.remaining_jobs.len();
    if n_remaining == 0 {
        return 0;
    }

    let mut job_costs: Vec<u64> = Vec::with_capacity(n_remaining);
    let mut min_r = u64::MAX;

    // 1. Dla każdego pozostałego zadania 'j' znajdź jego koszt
    for &job_j_id in &node.remaining_jobs {
        let task_j = &instance.tasks[job_j_id];

        // 1a. Znajdź najwcześniejszy moment gotowości spośród pozostałych
        if (task_j.ready_time as u64) < min_r {
            min_r = task_j.ready_time as u64;
        }

        // 1b. Znajdź minimalny czas przezbrojenia *do* zadania 'j'
        let mut min_s_j = u64::MAX;

        // Sprawdź przezbrojenie z ostatniego *zaplanowanego* zadania
        if let Some(last_id) = node.last_job_idx {
            // Zakładamy, że task.id == indeks
            min_s_j = instance.tasks[last_id].switch_time[job_j_id] as u64;
        }

        // Sprawdź przezbrojenie z dowolnego *innego pozostałego* zadania
        for &job_i_id in &node.remaining_jobs {
            if job_i_id != job_j_id {
                let s_ij = instance.tasks[job_i_id].switch_time[job_j_id] as u64;
                if s_ij < min_s_j {
                    min_s_j = s_ij;
                }
            }
        }

        // Jeśli nie ma poprzedników (np. to pierwszy węzeł), S_0j = 0
        if min_s_j == u64::MAX {
            min_s_j = 0;
        }

        // 1c. Zapisz koszt zadania jako (p_j + min S_ij)
        job_costs.push(task_j.processing_time as u64 + min_s_j);
    }

    // 2. Posortuj zrelaksowane koszty (logika SPT dla minimalizacji Sum C_j) [7]
    job_costs.sort();

    // 3. Oblicz skumulowany koszt Sum C_j dla tych zrelaksowanych zadań
    let mut remaining_cost_lb = 0;
    let mut cumulative_cost = 0;
    for cost in job_costs {
        cumulative_cost += cost;
        remaining_cost_lb += cumulative_cost;
    }

    // 4. Dodaj minimalny czas startu (uwzględniając r_j i czas bieżący)
    // dla wszystkich pozostałych zadań.
    let earliest_start = (node.current_time).max(min_r);
    remaining_cost_lb += n_remaining as u64 * earliest_start;

    remaining_cost_lb
}

/// Implementuje relaksację SRPT (Shortest Remaining Processing Time), ignorując S_ij.
/// Oblicza LB dla pozostałych zadań, gdy p_j są znaczące.
/// Używamy tu heurystyki SPT (nie-wywłaszczeniowej), która jest prostsza
/// w implementacji i wciąż stanowi poprawną dolną granicę (ponieważ S_ij >= 0).
fn lb_srpt_relaxation(node: &Node, instance: &Instance) -> u64 {
    debug!(
        "Calculating LB using SRPT relaxation for {} remaining jobs",
        node.remaining_jobs.len()
    );
    let n_remaining = node.remaining_jobs.len();
    if n_remaining == 0 {
        return 0;
    }

    // 1. Zbierz wszystkie pozostałe zadania
    let mut remaining_tasks: Vec<&Task> = node
        .remaining_jobs
        .iter()
        .map(|&id| &instance.tasks[id])
        .collect();

    // 2. Posortuj je zgodnie z regułą SPT (Shortest Processing Time) [8, 7]
    remaining_tasks.sort_by_key(|task| task.processing_time);

    // 3. Symuluj harmonogram (ignorując S_ij), aby obliczyć Sum C_j
    let mut current_time = node.current_time;
    let mut remaining_cost_lb = 0;

    for task in remaining_tasks {
        // Czas startu zadania to maksimum z jego czasu gotowości
        // lub czasu zakończenia poprzedniego zadania.
        let start_time = (task.ready_time as u64).max(current_time);

        // Czas zakończenia
        let completion_time = start_time + task.processing_time as u64;

        // Dodaj koszt C_j tego zadania do sumy LB
        remaining_cost_lb += completion_time;

        // Zaktualizuj czas dla następnego zadania w symulacji
        current_time = completion_time;
    }

    remaining_cost_lb
}
