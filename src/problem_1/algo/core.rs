use crate::problem_1::models::{Instance, InstanceMetrics};

pub fn analyze_instance(instance: Instance) -> InstanceMetrics {
    let n = instance.n;
    let avg_p = instance
        .tasks
        .iter()
        .map(|t| t.processing_time as f64)
        .sum::<f64>()
        / n as f64;

    let avg_s = instance
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
    InstanceMetrics {
        n,
        avg_s,
        avg_p,
        sts,
    }
}
