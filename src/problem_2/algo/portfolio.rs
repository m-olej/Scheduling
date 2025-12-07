use crate::problem_2::models::{Job, Machine, PriorityRule};

#[allow(non_camel_case_types)]
pub struct A_EDD {}

impl PriorityRule for A_EDD {
    fn name(&self) -> &str {
        "A-EDD"
    }

    fn calculate(&self, _t_current: f64, job: &Job, machine: &Machine) -> f64 {
        -job.d_j as f64
    }
}

#[allow(non_camel_case_types)]
pub struct A_SPT {}

impl PriorityRule for A_SPT {
    fn name(&self) -> &str {
        "A-SPT"
    }

    fn calculate(&self, _t_current: f64, job: &Job, machine: &Machine) -> f64 {
        -job.p_j as f64 * machine.b_k
    }
}

#[allow(non_camel_case_types)]
pub struct A_MDD {}

impl PriorityRule for A_MDD {
    fn name(&self) -> &str {
        "A-MDD"
    }

    fn calculate(&self, _t_current: f64, job: &Job, machine: &Machine) -> f64 {
        -job.r_j as f64
    }
}

#[allow(non_camel_case_types)]
pub struct ATC {}

impl PriorityRule for ATC {
    fn name(&self) -> &str {
        "ATC"
    }

    fn calculate(&self, t_current: f64, job: &Job, machine: &Machine) -> f64 {
        let slack = (job.d_j as f64 - t_current - job.p_j as f64 * machine.b_k).max(0.0);
        if slack == 0.0 {
            0.0
        } else {
            job.p_j as f64 * machine.b_k / slack
        }
    }
}

#[allow(non_camel_case_types)]
pub struct LS {}

impl PriorityRule for LS {
    fn name(&self) -> &str {
        "LS"
    }

    fn calculate(&self, _t_current: f64, job: &Job, machine: &Machine) -> f64 {
        -(job.r_j as f64 + job.p_j as f64 * machine.b_k)
    }
}
