use scheduling::problem_2::generate::Generator;
use scheduling::run_generator;

fn main() {
    env_logger::init();
    let generator = Generator {};
    run_generator(generator);
}
