use scheduling::problem_2::verify::Verifier;
use scheduling::run_verifier;

fn main() {
    env_logger::init();
    let verifier = Verifier {};
    run_verifier(verifier);
}
