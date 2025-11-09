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
        let mut best_score = original_score;
        let mut best_i: usize = 0;
        let mut best_j: usize = 0;

        for i in 0..n {
            for j in i + 1..n {
                schedule.task_ids.swap(i, j);
                let new_score = schedule.recalculate_score_from_index(i);
                // println!(
                //     "   Considering swap of positions {} and {}, new score: {}",
                //     i, j, new_score
                // );
                if new_score < best_score {
                    // println!(
                    //     "   Swap of positions {} and {} improves score from {} to {}",
                    //     i, j, original_score, new_score
                    // );
                    best_score = new_score;
                    best_i = i;
                    best_j = j;
                }
                schedule.task_ids.swap(i, j); // revert
            }
        }

        if best_score < original_score {
            // Apply the best found swap
            schedule.task_ids.swap(best_i, best_j);
            schedule.score = best_score;
            schedule.recalculate_score_from_index(best_i);
            return true;
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

        let original_score = schedule.score;
        let mut best_score = original_score;
        let mut best_task_idx: usize = 0;
        let mut best_insert_pos: usize = 0;

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
                if new_score < best_score {
                    // println!(
                    //     "   Relocate of task {} from pos {} to {} improves score from {} to {}",
                    //     task_id, task_idx, insert_pos, current_score, new_score
                    // );
                    best_score = new_score;
                    best_task_idx = task_idx;
                    best_insert_pos = insert_pos;
                }
                schedule.task_ids.remove(insert_pos);
            }
            schedule.task_ids.insert(task_idx, task_id); // revert
        }

        if best_score < original_score {
            // Apply the best found relocate
            let task_id = schedule.task_ids.remove(best_task_idx);
            schedule.task_ids.insert(best_insert_pos, task_id);
            schedule.score = best_score;
            schedule.recalculate_score_from_index(usize::min(best_task_idx, best_insert_pos));
            return true;
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

pub struct TwoOpt;
impl Neighborhood for TwoOpt {
    fn find_best_move(&self, schedule: &mut Schedule) -> bool {
        let n = schedule.task_ids.len();
        if n < 2 {
            return false;
        }

        let original_score = schedule.score;
        let mut best_score = original_score;
        let mut best_i: usize = 0;
        let mut best_j: usize = 0;

        for i in 0..n {
            for j in i + 1..n {
                // Zastosuj ruch: odwróć pod-sekwencję od i do j (włącznie)
                schedule.task_ids[i..=j].reverse();

                // Zmiana zaczyna się od indeksu `i`, więc przeliczamy od tego miejsca
                let new_score = schedule.recalculate_score_from_index(i);

                if new_score < best_score {
                    // Znaleziono pierwszą poprawę
                    best_score = new_score;
                    best_i = i;
                    best_j = j;
                }

                // Cofnij ruch: odwrócenie odwróconej sekwencji przywraca oryginał
                schedule.task_ids[i..=j].reverse();
            }
        }

        if best_score < original_score {
            // Zastosuj najlepszy znaleziony ruch
            schedule.task_ids[best_i..=best_j].reverse();
            schedule.score = best_score;
            schedule.recalculate_score_from_index(best_i);
            return true;
        }

        // Nie znaleziono poprawy w całym sąsiedztwie
        false
    }

    fn shake(&self, schedule: &mut Schedule, k: usize) {
        let mut rng = rng();
        let n = schedule.task_ids.len();
        if n < 2 {
            return;
        }

        println!("    Shaking with {} random 2-opt reversals", k);

        for _ in 0..k {
            let mut i = rng.random_range(0..n);
            let mut j = rng.random_range(0..n);

            // Upewnij się, że mamy dwa różne indeksy
            while j == i {
                j = rng.random_range(0..n);
            }

            // Upewnij się, że i < j
            if i > j {
                std::mem::swap(&mut i, &mut j);
            }

            schedule.task_ids[i..=j].reverse();
        }
    }
}

pub struct BlockMove;

// Definiuje maksymalny rozmiar bloku do przeniesienia.
// Rozmiar 1 jest już obsługiwany przez `Relocate`.
// Wartości 2 i 3 są typowe.
const MAX_BLOCK_SIZE: usize = 3;

impl Neighborhood for BlockMove {
    fn find_best_move(&self, schedule: &mut Schedule) -> bool {
        let n = schedule.task_ids.len();
        if n < 3 {
            // Potrzebujemy co najmniej 3 zadań, aby przenieść blok o rozmiarze 2
            return false;
        }

        let current_score = schedule.score;

        // Iteruj po różnych rozmiarach bloków (np. 2 i 3)
        for block_size in 2..=std::cmp::min(MAX_BLOCK_SIZE, n - 1) {
            // `i` to indeks początkowy bloku do usunięcia
            for i in 0..=n - block_size {
                // Krok 1: Usuń blok i zapisz go tymczasowo
                // Użycie `drain` jest wydajne
                let block: Vec<u32> = schedule.task_ids.drain(i..i + block_size).collect();

                // `j` to nowa pozycja wstawienia w skróconym harmonogramie
                // (n - block_size) to teraz nowy rozmiar `task_ids`
                for j in 0..=n - block_size {
                    // Krok 2: Wstaw blok w nowe miejsce
                    // `splice` efektywnie wstawia kolekcję
                    schedule.task_ids.splice(j..j, block.iter().cloned());

                    // Krok 3: Oblicz wynik
                    // Zmiana mogła nastąpić w `i` (usunięcie) lub `j` (wstawienie)
                    // Bezpiecznie jest przeliczyć od wcześniejszego z tych dwóch indeksów
                    let new_score = schedule.recalculate_score_from_index(std::cmp::min(i, j));

                    if new_score < current_score {
                        // Znaleziono pierwszą poprawę
                        schedule.score = new_score;
                        return true;
                    }

                    // Krok 4: Cofnij ruch (usuń wstawiony blok)
                    schedule.task_ids.drain(j..j + block_size);
                }

                // Krok 5: Cofnij ruch (wstaw blok z powrotem na oryginalne miejsce `i`)
                schedule.task_ids.splice(i..i, block.iter().cloned());
            }
        }

        false
    }

    fn shake(&self, schedule: &mut Schedule, k: usize) {
        let mut rng = rng();
        let n = schedule.task_ids.len();
        if n < 3 {
            return;
        }

        println!("    Shaking with {} random block moves", k);

        for _ in 0..k {
            // Wybierz losowy rozmiar bloku
            let block_size = rng.random_range(2..=std::cmp::min(MAX_BLOCK_SIZE, n - 1));

            // Wybierz losowe miejsce "skąd"
            let from_idx = rng.random_range(0..=n - block_size);

            // Usuń blok
            let block: Vec<u32> = schedule
                .task_ids
                .drain(from_idx..from_idx + block_size)
                .collect();

            // Wybierz losowe miejsce "dokąd" w nowym, krótszym wektorze
            let to_idx = rng.random_range(0..=schedule.task_ids.len());

            // Wstaw blok z powrotem
            schedule
                .task_ids
                .splice(to_idx..to_idx, block.iter().cloned());
        }
    }
}
