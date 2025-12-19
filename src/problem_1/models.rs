use log::debug;
use std::fmt::{Display, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub ready_time: u32,
    pub processing_time: u32,
    pub switch_time: Vec<u32>,
}

pub struct Instance {
    pub n: usize,
    pub tasks: Vec<Task>,
    pub metrics: InstanceMetrics,
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Instance(n: {})", self.n)?;
        for task in &self.tasks {
            writeln!(
                f,
                "  Task(id: {}, p_j: {}, r_j: {}, s_ij: {:?})",
                task.id, task.processing_time, task.ready_time, task.switch_time
            )?;
        }
        Ok(())
    }
}

impl Instance {
    pub fn read(file_content: &String) -> Self {
        let mut lines = file_content.lines();

        let n = lines.next().unwrap_or("0").parse().unwrap_or(0);
        debug!("Number of tasks (n): {}", n);

        let mut tasks = Vec::with_capacity(n);
        for (id, line) in lines.enumerate() {
            if id as u32 <= n as u32 - 1 {
                let mut parts = line.split_whitespace();
                let processing_time = parts.next().unwrap_or("0").parse().unwrap_or(0);
                let ready_time = parts.next().unwrap_or("0").parse().unwrap_or(0);
                let switch_time = vec![0; n];

                tasks.push(Task {
                    id: id as u32,
                    processing_time,
                    ready_time,
                    switch_time,
                });
            } else {
                let task_id = id - n;
                let mut parts = line.split_whitespace();
                for j in 0..n {
                    let setup_time = parts.next().unwrap_or("0").parse().unwrap_or(0);
                    tasks[task_id].switch_time[j] = setup_time;
                }
            }
        }

        return Self {
            n: n,
            tasks: tasks,
            metrics: InstanceMetrics {
                n: 0,
                avg_s: 0.0,
                avg_p: 0.0,
                sts: 0.0,
            },
        };
    }

    pub fn analyze(&mut self) {
        let n = self.n;
        let avg_p = self
            .tasks
            .iter()
            .map(|t| t.processing_time as f64)
            .sum::<f64>()
            / n as f64;

        let avg_s = self
            .tasks
            .iter()
            .map(|t| {
                t.switch_time
                    .iter()
                    .map(|st| if *st != t.id { *st as f64 } else { 0.0 })
                    .sum::<f64>()
            })
            .sum::<f64>()
            / (n * (n - 1)) as f64;

        let sts = avg_s / avg_p;
        self.metrics = InstanceMetrics {
            n,
            avg_s,
            avg_p,
            sts,
        };
    }

    pub fn format(&self) -> String {
        if self.n == 0 {
            return "0\n".to_string();
        }

        let mut output = String::with_capacity(self.n * self.n * 4);

        // Instance size
        writeln!(&mut output, "{}", self.n).unwrap();

        // Tasks (p_j r_j)
        for task in &self.tasks {
            writeln!(&mut output, "{} {}", task.processing_time, task.ready_time).unwrap();
        }

        // Setup times (s_ij)
        for i in 0..self.n {
            for j in 0..self.n {
                let setup_time = self.tasks[i].switch_time[j];
                write!(
                    &mut output,
                    "{}{}",
                    setup_time,
                    if j == self.n - 1 { "\n" } else { " " }
                )
                .unwrap();
            }
        }

        output
    }
}

#[derive(Debug)]
pub struct Solution<'a> {
    duration: u128,
    score: u64,
    pub tasks: Vec<&'a Task>, // (start_time, task_ref)
}

impl<'a> Solution<'a> {
    pub fn new(tasks: Vec<&'a Task>) -> Self {
        Self {
            duration: 0,
            score: 0,
            tasks: tasks,
        }
    }

    pub fn set_score(&mut self, score: u64) {
        self.score = score;
    }

    pub fn get_score(&self) -> u64 {
        self.score
    }

    pub fn set_duration(&mut self, duration: u128) {
        self.duration = duration;
    }

    pub fn get_duration(&self) -> u128 {
        self.duration
    }

    pub fn format(&self) -> String {
        let mut output = String::new();
        output += format!("{}\n", self.score).as_str();
        for task in &self.tasks {
            output += format!("{} ", task.id).as_str();
        }

        output
    }
}

#[derive(Clone)]
pub struct GlobalBest<'a> {
    pub solution: Arc<Mutex<Solution<'a>>>,
    pub score: Arc<AtomicU64>,
}

impl<'a> GlobalBest<'a> {
    /// Tworzy nowy współdzielony stan z początkowego rozwiązania.
    pub fn new(initial_solution: Solution<'a>) -> Self {
        let initial_score = initial_solution.score;
        Self {
            score: Arc::new(AtomicU64::new(initial_score)),
            solution: Arc::new(Mutex::new(initial_solution)),
        }
    }

    /// Szybka, bezblokadowa funkcja do odczytu najlepszego wyniku (UB).
    /// Będzie wywoływana tysiące razy na sekundę przez wątki do przycinania.
    pub fn get_best_score(&self) -> u64 {
        // `Ordering::Relaxed` jest najszybsze i w zupełności wystarczające
        // dla sprawdzania granicy w B&B.
        self.score.load(Ordering::Relaxed)
    }

    /// Funkcja wywoływana, gdy wątek znajdzie *potencjalnie* lepsze rozwiązanie.
    pub fn try_update_solution(&self, new_solution: Solution<'a>) {
        let new_score = new_solution.score;

        // Szybkie sprawdzenie atomowe (bez blokady)
        if new_score < self.get_best_score() {
            // Nasz wynik *może* być lepszy. Teraz musimy zdobyć blokadę,
            // aby zaktualizować pełne rozwiązanie.
            let mut solution_guard = self.solution.lock().unwrap();

            // Musimy sprawdzić ponownie *po* uzyskaniu blokady,
            // ponieważ inny wątek mógł nas ubiec, gdy czekaliśmy.
            if new_score < solution_guard.score {
                // Potwierdzone: nasz wynik jest nowym najlepszym.
                // Aktualizujemy pełne rozwiązanie wewnątrz Mutexa.
                *solution_guard = new_solution;

                // Na koniec aktualizujemy wartość atomową, aby wszystkie
                // inne wątki natychmiast zobaczyły nowy, lepszy wynik.
                self.score.store(new_score, Ordering::Relaxed);
            }
        }
        // Jeśli new_score >= current_best_score, nic nie robimy.
    }

    /// Funkcja do wywołania na samym końcu, aby pobrać finałowe rozwiązanie.
    pub fn into_inner(self) -> Solution<'a> {
        Arc::try_unwrap(self.solution)
            .expect("Powinien być jedynym właścicielem po zakończeniu wątków")
            .into_inner()
            .expect("Mutex nie powinien być zatruty (poisoned)")
    }
}

#[derive(Debug, Default)]
pub struct InstanceMetrics {
    pub n: usize,
    pub avg_s: f64, // avg processing time
    pub avg_p: f64, // avg ready time
    pub sts: f64,   // setup time severity
}

impl Display for InstanceMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Instance Metrics:\n - Number of tasks (n): {}\n - Average processing time (avg_p): {:.2}\n - Average switch time (avg_s): {:.2}\n - Switch Time Severity (sts): {:.4}",
            self.n, self.avg_p, self.avg_s, self.sts
        )
    }
}
