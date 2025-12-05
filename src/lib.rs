use clap::{command, Parser};
use log::info;
use std::error::Error;
use std::path::{Path, PathBuf};
pub mod file_handler;
pub mod problem_2;

///
/// Library generic traits
///

/// A generic result type for your library
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Data Model Traits (for loading/saving)
pub trait SchedulableProblem: Sized {
    fn from_file(path: &Path) -> Result<Self>;
    fn to_file(&self, path: &Path) -> Result<()>;
}

pub trait SchedulableSolution: Sized {
    type Problem: SchedulableProblem;
    fn calculate_score(&self, instance: &Self::Problem) -> i64;
    fn from_file(path: &Path) -> Result<Self>;
    fn to_file(&self, path: &Path) -> Result<()>;
}

/// Behavioral Traits
pub trait ProblemGenerator {
    type Problem: SchedulableProblem;
    /// Generate a new instance
    fn generate(&self, size: usize, seed: u64) -> Self::Problem;
}

pub trait ProblemVerifier {
    type Problem: SchedulableProblem;
    type Solution: SchedulableSolution;

    /// Validates if solution to an instance is valid
    fn verify_solution(&self, problem: &Self::Problem, solution: &Self::Solution) -> bool;

    /// Validates if the instance is valid
    fn verify_instance(&self, instance: &Self::Problem) -> bool;
}

pub trait ProblemSolver<'a> {
    type Problem: SchedulableProblem;
    type Solution: SchedulableSolution;

    fn solve(&self, problem: &mut Self::Problem) -> Self::Solution;
}

/// Drivers

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct SolverArgs {
    /// Instance input file path
    #[arg(short, long)]
    input_instance: PathBuf,

    /// Solution output file path
    #[arg(short, long)]
    output_file: PathBuf,
}

/// Generator program for scheduling problems
///
/// This function runs a generator implementation that creates problem instances
#[derive(Parser)]
#[command(version, about, long_about)]
struct GeneratorArgs {
    /// Size of the instance to generate (number of jobs)
    #[arg(short, long)]
    size: usize,
    /// Output directory
    #[arg(short, long)]
    output_dir: PathBuf,
    /// Optional seed argument
    seed: Option<u64>,
}

/// Verifier program for scheduling problems
///
/// Can check validity of instances and solutions of a given scheduling problem
#[derive(Parser)]
#[command(version, about, long_about)]
struct VerifierArgs {
    /// path to the problem instance file
    #[arg(short, long)]
    instance_file: PathBuf,
    /// path to the solution file
    solution_file: Option<PathBuf>,
}

pub fn run_generator<G>(generator_implementation: G)
where
    G: ProblemGenerator,
{
    let args = GeneratorArgs::parse();
    // handle optional seed
    let seed = match args.seed {
        Some(seed) => seed,
        None => rand::random(),
    };

    println!(
        "Running generator with size: {}, seed: {}, output: {:?}",
        args.size, seed, args.output_dir
    );

    let instance = generator_implementation.generate(args.size, seed);
    instance
        .to_file(&args.output_dir)
        .expect("Failed to save generated instance to file");
    println!("Generated instance saved to {:?}", args.output_dir);
}

pub fn run_verifier<V>(verifier_implementation: V)
where
    V: ProblemVerifier,
{
    let args = VerifierArgs::parse();
    println!(
        "Running verifier with instance: {:?}, solution: {:?}",
        args.instance_file, args.solution_file
    );

    // Load problem
    let problem =
        V::Problem::from_file(&args.instance_file).expect("Failed to load problem from file");

    // Load solution if provided
    match args.solution_file {
        Some(ref solution_file) => {
            // check both instance and solution validity
            let solution =
                V::Solution::from_file(solution_file).expect("Failed to load solution from file");
            if verifier_implementation.verify_solution(&problem, &solution) {
                println!("Both instance and solution are valid");
            } else {
                println!("Instance or solution is invalid");
            }
        }
        None => {
            // check only instance validity
            if verifier_implementation.verify_instance(&problem) {
                println!("Instance is valid");
            } else {
                println!("Instance is invalid");
            }
            return;
        }
    };
}

pub fn run_solver<S>(solver_implementation: S)
where
    S: for<'a> ProblemSolver<'a>,
{
    let args = SolverArgs::parse();
    info!(
        "Running solver with input: {:?}, output: {:?}",
        args.input_instance, args.output_file
    );

    // Load problem
    let mut problem =
        S::Problem::from_file(&args.input_instance).expect("Failed to load problem from file");

    // Solve problem
    let solution = solver_implementation.solve(&mut problem);

    // Save solution
    solution
        .to_file(&args.output_file)
        .expect("Failed to save solution to file");

    info!("Solution saved to {:?}", args.output_file);
}
