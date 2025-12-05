use crate::file_handler::{read_from_file, write_to_file};
use crate::{Result, SchedulableProblem, SchedulableSolution};
use log::debug;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;

/// Standardization Factor for floating point to integer conversion
pub const STANDARDIZATION_FACTOR: i64 = 10;

/// Immutable copy of the problem instance data
/// Clone is cheap if we wrap Vec<Job> in Arc
#[derive(Clone, Copy)]
pub struct Job {
    pub id: usize,
    pub p_j: i64, // Czas bazowy (w skali, np. * 1000)
    pub r_j: i64, // Czas gotowości (w skali)
    pub d_j: i64, // Termin zakończenia (w skali)
}

/// Machine capable of processing jobs
#[derive(Clone, Copy)]
pub struct Machine {
    /// machine identifier
    pub id: usize,
    /// slowdown factor
    pub b_k: i64,
}

/// Result of scheduling a job on a machine
#[derive(Clone)]
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

/// Priority rule trait for job scheduling
pub trait PriorityRule: Send + Sync {
    fn name(&self) -> &str;
    /// Calculate priority of a job at current time
    fn calculate(&self, t_current: i64, job: &Job) -> i64;
}

#[derive(Clone)]
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
                        b_k: (part.parse::<f32>().unwrap() * STANDARDIZATION_FACTOR as f32).trunc()
                            as i64,
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
            content.push_str(&format!(
                "{} ",
                machine.b_k as f32 / STANDARDIZATION_FACTOR as f32
            ));
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
    type Problem = Instance;

    fn calculate_score(&self, instance: &Self::Problem) -> i64 {
        let mut total_tardy_work: f64 = 0.0;
        let mut machine_times: HashMap<usize, i64> = HashMap::new();

        debug!(
            "Calculating score for solution with {} job results",
            self.job_results.len()
        );

        for result in &self.job_results {
            let job = &instance.jobs[result.job_id - 1];
            debug!(
                "Processing Job {} on Machine {}: p_j={}, r_j={}, d_j={}",
                job.id, result.machine_id, job.p_j, job.r_j, job.d_j
            );
            let completion_time;
            if machine_times.contains_key(&result.machine_id) {
                completion_time = job.r_j.max(machine_times[&result.machine_id])
                    + job.p_j * instance.machines[result.machine_id].b_k / STANDARDIZATION_FACTOR;
                machine_times.insert(result.machine_id, completion_time);
            } else {
                completion_time = job.r_j
                    + job.p_j * instance.machines[result.machine_id].b_k / STANDARDIZATION_FACTOR;
                machine_times.insert(result.machine_id, completion_time);
            }
            debug!(
                "Job {} completion time on Machine {}: {}",
                job.id, result.machine_id, completion_time
            );

            let b_k: f64 =
                instance.machines[result.machine_id].b_k as f64 / STANDARDIZATION_FACTOR as f64;
            let tardy_work: f64 =
                (job.p_j as f64 * b_k).min((completion_time - job.d_j).max(0) as f64) / b_k;
            debug!(
                "Job {} tardy work on Machine {}: {}",
                job.id, result.machine_id, tardy_work
            );
            total_tardy_work += tardy_work
        }

        debug!("Total tardy work (score): {}", total_tardy_work);

        total_tardy_work.trunc() as i64
    }

    fn from_file(path: &Path) -> Result<Self> {
        // Read file content
        let content = match read_from_file(path) {
            Ok(data) => data,
            Err(err) => panic!("Error reading file: {}", err),
        };

        // Parse content
        let lines = content.lines();
        let mut score = 0;
        let mut job_results: Vec<JobResult> = Vec::new();

        for (i, line) in lines.enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if i == 0 {
                score = trimmed.parse().unwrap();
            } else {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                for s in parts {
                    job_results.push(JobResult {
                        job_id: s.parse().unwrap(),
                        machine_id: i - 1,
                        completion_time: 0,
                        tardy_work: 0,
                    });
                }
            }
        }

        Ok(Solution {
            strategy: "unknown".to_string(),
            score,
            job_results,
        })
    }

    fn to_file(&self, path: &Path) -> Result<()> {
        let mut content = String::new();
        content.push_str(&format!("{}\n", self.score));
        let mut machine_results: HashMap<usize, Vec<usize>> = HashMap::new();
        for result in &self.job_results {
            machine_results
                .entry(result.machine_id)
                .or_insert_with(Vec::new)
                .push(result.clone().job_id);
        }
        for machine_id in 0..machine_results.len() {
            for job_id in &machine_results[&machine_id] {
                content.push_str(&format!("{} ", job_id));
            }
            content.push_str("\n");
        }
        write_to_file(path, &content);
        Ok(())
    }
}
