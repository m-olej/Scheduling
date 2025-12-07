use crate::problem_2::models::{Instance, Job, Machine};
use crate::ProblemGenerator;
use rand::rand_core::block::BlockRng;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Core;
use std::iter;
pub struct Generator {}

const MAX_B_K: f32 = 2.0;
const MIN_B_K: f32 = 1.0;
const MIN_P_J: i64 = 1;
const MAX_P_J: i64 = 1000;
const MIN_R_J: i64 = 0;
const MAX_R_J: i64 = 500;
const MIN_D_J: i64 = 100;
const MAX_D_J: i64 = 600;

impl ProblemGenerator for Generator {
    type Problem = Instance;

    fn generate(&self, size: usize, seed: u64) -> Instance {
        let n = size;
        let m = 5;

        let mut rng: BlockRng<ChaCha8Core> = BlockRng::seed_from_u64(seed);

        let jobs: Vec<Job> = (0..n)
            .map(|i| {
                let r_j = rng.random_range(MIN_R_J..MAX_R_J);
                let p_j = rng.random_range(MIN_P_J..MAX_P_J);
                let d_j = rng.random_range((r_j + p_j + MIN_D_J)..(r_j + p_j + MAX_D_J));
                Job {
                    id: i,
                    p_j,
                    r_j,
                    d_j,
                }
            })
            .collect();

        let machines: Vec<Machine> = iter::once(Machine { id: 0, b_k: 1.0 })
            .chain((1..m).map(|i| Machine {
                id: i,
                b_k: (rng.random_range(MIN_B_K..MAX_B_K)).trunc() as f64,
            }))
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
