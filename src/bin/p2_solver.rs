use scheduling::problem_2::solve::Solver;
use scheduling::run_solver;

fn main() {
    env_logger::init();
    let solver = Solver {};
    run_solver(solver);
}
