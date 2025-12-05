use crate::problem_2::algo::des::*;
use crate::problem_2::algo::portfolio::*;
use crate::problem_2::models::*;
use crate::ProblemSolver;
use log::info;
use rayon::prelude::*;
use std::sync::Arc;

pub struct Solver {}

impl ProblemSolver<'_> for Solver {
    type Problem = Instance;
    type Solution = Solution;

    fn solve(&self, instance: &mut Self::Problem) -> Self::Solution {
        let jobs_arc = Arc::new(instance.jobs.clone());
        let machines_arc = Arc::new(instance.machines.clone());

        // 1. Zbuduj portfel strategii
        let portfolio: Vec<Box<dyn PriorityRule>> = vec![
            Box::new(A_EDD {}),
            Box::new(A_SPT {}),
            Box::new(A_MDD {}),
            Box::new(ATC {}),
            Box::new(LS {}),
        ];

        // 2. Uruchom wszystkie heurystyki równolegle
        let results: Vec<ScheduleResult> = portfolio
            .par_iter()
            .map(|rule| {
                // Klonowanie Arc jest tanie
                let jobs_clone = Arc::clone(&jobs_arc);
                let machines_clone = Arc::clone(&machines_arc);

                // Każdy wątek wykonuje pełną, niezależną symulację
                run_simulation(&jobs_clone, &machines_clone, rule.as_ref())
            })
            .collect();

        // 3. Znajdź najlepszy deterministyczny wynik
        let best_result = results
            .iter()
            .min_by_key(|res| res.total_tardy_work)
            .expect("Symulacje nie dały żadnych wyników");

        info!("Najlepsza reguła: {}", best_result.rule_name);
        info!(
            "Całkowita praca spóźniona: {}",
            best_result.total_tardy_work
        );

        Solution {
            strategy: best_result.rule_name.clone(),
            score: best_result.total_tardy_work,
            job_results: best_result.schedule.clone(),
        }
    }
}
