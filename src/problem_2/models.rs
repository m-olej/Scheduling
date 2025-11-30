use crate::file_handler::{read_from_file, write_to_file};
use crate::{Result, SchedulableProblem, SchedulableSolution};
use std::cmp::Ordering;
use std::path::Path;

/// Immutable copy of the problem instance data
/// Clone is cheap if we wrap Vec<Job> in Arc
#[derive(Clone)]
pub struct Job {
    pub id: usize,
    pub p_j: i64, // Czas bazowy (w skali, np. * 1000)
    pub r_j: i64, // Czas gotowości (w skali)
    pub d_j: i64, // Termin zakończenia (w skali)
}

/// Machine capable of processing jobs
pub struct Machine {
    /// machine identifier
    pub id: usize,
    /// slowdown factor
    pub b_k: i64,
}

/// Result of scheduling a job on a machine
pub struct JobResult {
    /// job identifier
    pub job_id: usize,
    /// machine identifier
    pub machine_id: usize,
    /// total elapsed time after job completion on the machine
    pub completion_time: i64,
    /// tardiness of the job
    pub tardy_work: i64,
}

/// Current state of a machine in the heap
#[derive(Eq, PartialEq)]
pub struct MachineState {
    /// machine identifier
    pub machine_id: usize,
    /// time when the machine becomes free
    pub free_time: i64,
}

/// Min-heap based on machine free_time
impl Ord for MachineState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .free_time
            .cmp(&self.free_time)
            .then_with(|| self.machine_id.cmp(&other.machine_id))
    }
}

impl PartialOrd for MachineState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Instance {
    /// number of jobs
    pub n: usize,
    /// number of machines
    pub m: usize,
    /// list of jobs
    pub jobs: Vec<Job>,
    /// list of machines
    pub machines: Vec<Machine>,
}

pub struct Solution {
    /// used strategy for scheduling
    pub strategy: String,
    /// total score of the solution
    pub score: i64,
    /// results of each scheduled job
    pub job_results: Vec<JobResult>,
}

impl SchedulableProblem for Instance {
    fn from_file(path: &Path) -> Result<Self> {
        // Read file content
        let content = match read_from_file(path) {
            Ok(data) => data,
            Err(err) => panic!("Error reading file: {}", err),
        };

        // Parse content
        let lines = content.lines();
        let mut n = 0;
        let mut m = 0;
        let mut jobs = Vec::new();
        let mut machines = Vec::new();
        let mut job_id = 0;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if n == 0 {
                n = trimmed.parse().unwrap();
            } else if m == 0 {
                let machine_line_parts: Vec<&str> = trimmed.split_whitespace().collect();

                m = machine_line_parts.len();
                for (i, part) in machine_line_parts.iter().enumerate() {
                    let machine = Machine {
                        id: i,
                        b_k: part.parse().unwrap(),
                    };
                    machines.push(machine);
                }
            } else {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                let job = Job {
                    id: job_id,
                    p_j: parts[0].parse().unwrap(),
                    r_j: parts[1].parse().unwrap(),
                    d_j: parts[2].parse().unwrap(),
                };
                jobs.push(job);
                job_id += 1;
            }
        }

        Ok(Instance {
            n,
            m,
            jobs,
            machines,
        })
    }

    fn to_file(&self, path: &Path) -> Result<()> {
        let mut content = String::new();
        content.push_str(&format!("{}\n", self.n));
        for machine in &self.machines {
            content.push_str(&format!("{} ", machine.b_k));
        }
        content.push_str("\n");
        for job in &self.jobs {
            content.push_str(&format!("{} {} {}\n", job.p_j, job.r_j, job.d_j));
        }
        write_to_file(path, &content);
        Ok(())
    }
}

impl SchedulableSolution for Solution {
    fn calculate_score(&self) -> i64 {
        let mut total_tardy_work = 0;
        for result in &self.job_results {
            total_tardy_work += result.tardy_work;
        }
        total_tardy_work
    }

    fn from_file(path: &Path) -> Result<Self> {
        // Read file content
        let content = match read_from_file(path) {
            Ok(data) => data,
            Err(err) => panic!("Error reading file: {}", err),
        };

        // Parse content
        let lines = content.lines();
        let mut strategy = String::new();
        let mut score = 0;
        let mut job_results = Vec::new();

        for (i, line) in lines.enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if i == 0 {
                strategy = trimmed.replace("Strategy: ", "");
            } else if i == 1 {
                let score_part = trimmed.replace("Score: ", "");
                score = score_part.parse().unwrap();
            } else {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                let job_result = JobResult {
                    job_id: parts[1].parse().unwrap(),
                    machine_id: parts[4].parse().unwrap(),
                    completion_time: parts[7].parse().unwrap(),
                    tardy_work: parts[10].parse().unwrap(),
                };
                job_results.push(job_result);
            }
        }

        Ok(Solution {
            strategy,
            score,
            job_results,
        })
    }

    fn to_file(&self, path: &Path) -> Result<()> {
        let mut content = String::new();
        content.push_str(&format!("Strategy: {}\n", self.strategy));
        content.push_str(&format!("Score: {}\n", self.score));
        for result in &self.job_results {
            content.push_str(&format!(
                "Job {} on Machine {}: Completion Time {}, Tardy Work {}\n",
                result.job_id, result.machine_id, result.completion_time, result.tardy_work
            ));
        }
        write_to_file(path, &content);
        Ok(())
    }
}
