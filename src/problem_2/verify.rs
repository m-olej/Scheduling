use crate::problem_2::models::{Instance, Solution};
use crate::ProblemVerifier;
use crate::SchedulableSolution;
use ::log::{error, warn};

pub struct Verifier {}

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
        if machines[0].b_k != 10 {
            error!(
                "Machine 0 has invalid b_k value: {} (expected 10)",
                machines[0].b_k
            );
            return false;
        }
        for machine in &machines[1..] {
            if machine.b_k < 10 || machine.b_k > 20 {
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
        if calculated_score != solution.score {
            println!(
                "Solution score mismatch: calculated {}, but solution has {}",
                calculated_score, solution.score
            );
            return false;
        }

        // Scheduling validity
        let n = instance.n;
        let jobs = &instance.jobs;
        let job_results = &solution.job_results;

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
            let job_start_time = job_result.completion_time - job.p_j;
            // Start time >= release time
            if job_start_time < job.r_j {
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
