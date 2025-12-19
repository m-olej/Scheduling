use crate::problem_1::algo::bounds::BoundingStrategy;
use crate::problem_1::models::{Instance, Solution, Task};
use log::debug;
use std::collections::HashSet;

pub fn calculate_score(sequence: &Vec<&Task>) -> (u64, u128) {
    let mut total_cost = 0_u64;
    let mut last_completion_time = 0_u64;
    let mut last_task: Option<&Task> = None;

    for &task in sequence {
        let p_j = task.processing_time as u64;
        let r_j = task.ready_time as u64;

        // Pobiera S_ij z 'switch_time' poprzedniego zadania
        let s_ij = match last_task {
            Some(prev_task) => prev_task.switch_time[task.id as usize] as u64,
            None => 0, // S_0j, zakładamy 0 dla pierwszego zadania
        };

        // Rekurencyjna formuła: C_j = max(r_j, C_{i} + S_ij) + p_j
        let start_time = (last_completion_time + s_ij).max(r_j);
        let completion_time = start_time + p_j;

        total_cost += completion_time; // Dodajemy C_j do sumy
        last_completion_time = completion_time;
        last_task = Some(task);
    }

    (total_cost, last_completion_time as u128)
}

pub fn find_initial_solution<'a>(
    instance: &'a Instance,
    solution: &mut Solution<'a>,
    strategy: &BoundingStrategy,
) {
    match strategy {
        BoundingStrategy::ATCS => heuristic_insertion(instance, solution),
        BoundingStrategy::INSERT => heuristic_acts(instance, solution),
        BoundingStrategy::HYBRID => heuristic_insertion(instance, solution),
    };

    let (initial_score, initial_duration) = calculate_score(&solution.tasks);

    debug!(
        "Initial solution found using {:?} with score: {}",
        strategy, initial_score
    );

    solution.set_score(initial_score);
    solution.set_duration(initial_duration);
}

fn heuristic_insertion<'a>(instance: &'a Instance, solution: &mut Solution<'a>) {
    debug!("Uruchamianie heurystyki: Najlepsze Wstawianie (Insertion)...");
    solution.tasks.clear(); // Zaczynamy od pustej sekwencji

    // Pobieramy referencje do wszystkich zadań
    let mut unscheduled_tasks: Vec<&'a Task> = instance.tasks.iter().collect();

    if unscheduled_tasks.is_empty() {
        return;
    }

    // 1. Zacznij od zadania z najwcześniejszym r_j (momentem gotowości)
    unscheduled_tasks.sort_by_key(|t| t.ready_time);
    let first_task = unscheduled_tasks.remove(0);
    solution.tasks.push(first_task);

    // 3. W pętli (dla n-1 zadań):
    while !unscheduled_tasks.is_empty() {
        let mut best_task_index = 0;
        let mut best_position = 0;
        let mut min_resulting_cost = u64::MAX;

        // 4. Iteruj po wszystkich nieużytych zadaniach (k).
        for (task_idx, &task_to_insert) in unscheduled_tasks.iter().enumerate() {
            // 5. Iteruj po wszystkich możliwych pozycjach wstawienia (p).
            for pos in 0..=solution.tasks.len() {
                let mut temp_sequence = solution.tasks.clone();
                temp_sequence.insert(pos, task_to_insert);

                // 6. Oblicz *pełny* koszt nowej sekwencji.
                // Kluczowa adaptacja dla Sum C_j: minimalizujemy całkowity koszt,
                // a nie tylko koszt lokalnego wstawienia.
                let (cost, _duration) = calculate_score(&temp_sequence);

                // 7. Wybierz parę (k*, p*), która minimalizuje Sum C_j.
                if cost < min_resulting_cost {
                    min_resulting_cost = cost;
                    best_task_index = task_idx;
                    best_position = pos;
                }
            }
        }

        // 8. Wstaw k* na pozycję p*.
        let chosen_task = unscheduled_tasks.remove(best_task_index);
        solution.tasks.insert(best_position, chosen_task);
    }
}

fn heuristic_acts<'a>(instance: &'a Instance, solution: &mut Solution<'a>) {
    debug!("Uruchamianie heurystyki: Reguła Pierwszeństwa (ACTS)...");
    solution.tasks.clear(); // Zaczynamy od pustej sekwencji

    // Używamy HashSet ID zadań dla szybkiego usuwania O(1)
    let mut unscheduled_task_ids: HashSet<u32> = (0..instance.n as u32).collect();

    // Parametry skalujące dla reguły ACTS [1, 2]
    let k_setup = 2.0; // Typowy parametr K dla przezbrojeń
    let avg_s = instance.metrics.avg_s;

    let mut current_time = 0_u64;
    let mut last_task: Option<&'a Task> = None;

    // 2. W pętli (n razy):
    while solution.tasks.len() < instance.n {
        // 3. Znajdź dostępne zadania (r_j <= current_time)
        let available_task_ids: Vec<u32> = unscheduled_task_ids
            .iter()
            .filter(|&&id| (instance.tasks[id as usize].ready_time as u64) <= current_time)
            .cloned()
            .collect();

        let best_task_id: u32;

        if available_task_ids.is_empty() {
            // --- Czas bezczynności ---
            // Żadne zadanie nie jest gotowe. Przesuń czas do przodu.
            let min_r = unscheduled_task_ids
                .iter()
                .map(|&id| instance.tasks[id as usize].ready_time as u64)
                .min()
                .unwrap_or(current_time);

            current_time = min_r.max(current_time); // Przeskocz do przodu

            // Ponownie znajdź dostępne zadania. Tym razem musi jakieś być.
            // Używamy prostej reguły SPT (Shortest Processing Time) do wyboru
            // spośród zadań, które właśnie stały się dostępne.
            best_task_id = unscheduled_task_ids
                .iter()
                .filter(|&&id| (instance.tasks[id as usize].ready_time as u64) <= current_time)
                .min_by_key(|&&id| instance.tasks[id as usize].processing_time)
                .cloned()
                .unwrap(); //.unwrap() jest bezpieczne, bo unscheduled_task_ids nie jest puste
        } else {
            // --- Wybór zadania ---
            // 5. Oblicz dla każdego z nich indeks ACTS (adaptacja ATCS bez terminów)
            //    I_j(t, i) = (1 / p_j) * exp(-S_ij / (K * avg_s)) [1, 4, 6]

            let (best_id, _) = available_task_ids
                .iter()
                .map(|&task_id| {
                    let task = &instance.tasks[task_id as usize];
                    let p_j_f = task.processing_time as f64;

                    let s_ij_f = match last_task {
                        Some(lt) => lt.switch_time[task.id as usize] as f64,
                        None => 0.0,
                    };

                    // Unikamy dzielenia przez zero, jeśli p_j jest 0
                    if p_j_f == 0.0 {
                        return (task_id, f64::MAX); // Dajemy priorytet zadaniom o zerowym czasie
                    }

                    let setup_component = if avg_s > 0.001 {
                        (-s_ij_f / (k_setup * avg_s)).exp()
                    } else {
                        1.0 // Ignoruj komponent przezbrojenia
                    };

                    let index = (1.0 / p_j_f) * setup_component;
                    (task_id, index)
                })
                .max_by(|(_, index_a), (_, index_b)| {
                    index_a
                        .partial_cmp(index_b)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap(); //.unwrap() jest bezpieczne, bo available_tasks nie jest puste

            best_task_id = best_id;
        }

        // 7. Dodaj wybrane zadanie do sekwencji i zaktualizuj stan
        let chosen_task = &instance.tasks[best_task_id as usize];

        let p_j = chosen_task.processing_time as u64;
        let r_j = chosen_task.ready_time as u64;
        let s_ij = match last_task {
            Some(lt) => lt.switch_time[chosen_task.id as usize] as u64,
            None => 0,
        };

        let start_time = (current_time + s_ij).max(r_j);
        current_time = start_time + p_j;
        last_task = Some(chosen_task);

        solution.tasks.push(chosen_task);
        unscheduled_task_ids.remove(&chosen_task.id);
    }
}
