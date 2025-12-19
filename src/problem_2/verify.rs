use crate::problem_2::models::{Instance, JobResult, Solution};
use crate::ProblemVerifier;
use crate::SchedulableSolution;
use ::log::{debug, error, warn};

pub struct Verifier {}

fn calculate_completion_times(instance: &Instance, job_results: &Vec<JobResult>) -> Vec<JobResult> {
    let mut machine_times: Vec<f64> = vec![0.0; instance.m];
    let mut filled_job_results = job_results.clone();

    for result in &mut filled_job_results {
        let job = &instance.jobs[result.job_id];
        let machine = &instance.machines[result.machine_id];

        let start_time = machine_times[result.machine_id].max(job.r_j as f64);
        let finish_time = start_time + job.p_j as f64 * machine.b_k;

        machine_times[result.machine_id] = finish_time;
        result.completion_time = finish_time;
    }

    filled_job_results
}

impl ProblemVerifier for Verifier {
    type Problem = Instance;
    type Solution = Solution;

    fn verify_instance(&self, instance: &Self::Problem) -> bool {
        let n = instance.n;
        let m = instance.m;
        let jobs = &instance.jobs;
        let machines = &instance.machines;

        // Machine count
        if machines.len() != m {
            error!("Expected {} machines, but found {}", m, machines.len());
            return false;
        }
        // Machine attributes (b_k) format
        let mut machine_bk_check = false;
        for machine in machines {
            if machine.b_k == 1.0 {
                machine_bk_check = true;
            }
        }
        if !machine_bk_check {
            error!("At least one machine has b_k not equal to 1.0");
            return false;
        }

        for machine in &machines[1..] {
            if machine.b_k < 1.0 || machine.b_k > 2.0 {
                error!(
                    "Machine {} has invalid b_k value: {}",
                    machine.id, machine.b_k
                );
                return false;
            }
        }

        // Job count
        if jobs.len() != n {
            error!("Expected {} jobs, but found {}", n, jobs.len());
            return false;
        }
        // Job attributes (p_j, r_j, d_j) non-negative
        for job in jobs {
            if job.p_j < 0 || job.r_j < 0 || job.d_j < 0 {
                error!("Job {} has negative attribute(s)", job.id);
                return false;
            }
        }
        // Optional: check if there are impossible jobs (d_j < r_j + p_j)
        for job in jobs {
            if job.d_j < job.r_j + job.p_j {
                warn!(
                    "Job {} is impossible to schedule: d_j < r_j + p_j ({} < {} + {})",
                    job.id, job.d_j, job.r_j, job.p_j
                );
            }
        }

        true
    }

    fn verify_solution(&self, instance: &Self::Problem, solution: &Self::Solution) -> bool {
        // instance
        if !self.verify_instance(instance) {
            println!("Instance verification failed");
            return false;
        }

        // score
        let calculated_score = solution.calculate_score(instance);
        if calculated_score != solution.score.round().trunc() as i64 {
            println!(
                "Solution score mismatch: calculated {}, but solution has {}",
                calculated_score,
                solution.score.round().trunc() as i64
            );
            return false;
        }

        // Scheduling validity
        let n = instance.n;
        let jobs = &instance.jobs;
        let job_results = calculate_completion_times(instance, &solution.job_results);

        // Job count
        if job_results.len() != n {
            println!(
                "Expected {} job results, but found {}",
                n,
                job_results.len()
            );
            return false;
        }

        // Job scheduling attributes validity
        for job_result in job_results {
            let job = &jobs[job_result.job_id];
            let duration = job.p_j as f64 * instance.machines[job_result.machine_id].b_k;
            let job_start_time = job_result.completion_time as f64 - duration;
            debug!(
                "Verifying Job {} on Machine {}: duration {} start_time {} completion_time {}",
                job.id, job_result.machine_id, duration, job_start_time, job_result.completion_time
            );
            // Start time >= release time
            if job_start_time.round() < job.r_j as f64 {
                println!(
                    "Job {} starts before its release time: start_time {} < r_j {}",
                    job.id, job_start_time, job.r_j
                );
                return false;
            }
        }

        true
    }
}
