use crate::problem_2::models::{Instance, Job, Machine};
use crate::ProblemGenerator;
use rand::{rng, Rng};

pub struct Generator {
    pub size: usize,
    /// Optional seed for reproducibility (NotImplemented)
    pub seed: Option<u64>,
}

const MAX_B_K: f32 = 2.0;
const MIN_B_K: f32 = 1.0;
const MIN_P_J: i64 = 1;
const MAX_P_J: i64 = 1000;
const MIN_R_J: i64 = 0;
const MAX_R_J: i64 = 500;
const MIN_D_J: i64 = 100;
const MAX_D_J: i64 = 5000;

impl ProblemGenerator for Generator {
    type Problem = Instance;

    fn generate(&self, size: usize, seed: u64) -> Instance {
        let n = size;
        let m = 5;

        let mut rng = rng();

        let jobs: Vec<Job> = (0..n)
            .map(|i| {
                let r_j = rng.gen_range(MIN_R_J..MAX_R_J);
                let p_j = rng.gen_range(MIN_P_J..MAX_P_J);
                let d_j = rng.gen_range((r_j + p_j + MIN_D_J)..(r_j + p_j + MAX_D_J));
                Job {
                    id: i,
                    p_j,
                    r_j,
                    d_j,
                }
            })
            .collect();

        let machines: Vec<Machine> = (0..m)
            .map(|i| Machine {
                id: i,
                b_k: rng.gen_range(MIN_B_K..MAX_B_K),
            })
            .collect();

        // Dummy implementation
        Instance {
            n,
            m,
            jobs,
            machines,
        }
    }
}
