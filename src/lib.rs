use clap::{command, Parser};
use std::error::Error;
use std::path::{Path, PathBuf};
pub mod file_handler;
pub mod problem_1;
pub mod problem_2;

///
/// Library generic traits
///

/// A generic result type for your library
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Data Model Traits (for loading/saving)
/// We assume all your problems will need to be serializable/deserializable
pub trait SchedulableProblem: Sized {
    fn from_file(path: &Path) -> Result<Self>;
    fn to_file(&self, path: &Path) -> Result<()>;
}

pub trait SchedulableSolution: Sized {
    fn calculate_score(&self) -> i64;
    fn from_file(path: &Path) -> Result<Self>;
    fn to_file(&self, path: &Path) -> Result<()>;
}

/// Behavioral Traits
pub trait ProblemGenerator {
    type Problem: SchedulableProblem;
    /// Generate a new instance
    fn generate(&self, size: usize, seed: u64) -> Self::Problem;
}

pub trait Verifier {
    type Problem: SchedulableProblem;
    type Solution: SchedulableSolution;

    /// Return true if valid, false otherwise
    fn verify(&self, problem: &Self::Problem, solution: &Self::Solution) -> bool;
}

pub trait Solver<'a> {
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

    /// Solution output directory
    #[arg(short, long)]
    output_dir: PathBuf,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct GeneratorArgs {
    /// Size of the instance to generate
    #[arg(short, long)]
    size: usize,

    /// Output directory
    #[arg(short, long)]
    output_dir: PathBuf,

    /// Optional seed argument
    seed: Option<u64>,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct VerifierArgs {
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    output: PathBuf,
}

pub fn run_solver<S>(solver_implementation: S)
where
    S: for<'a> Solver<'a>,
{
    let args = SolverArgs::parse();
    println!(
        "Running solver with input: {:?}, output: {:?}",
        args.input_instance, args.output_dir
    );

    // Load problem
    let mut problem =
        S::Problem::from_file(&args.input_instance).expect("Failed to load problem from file");

    // Solve problem
    let solution = solver_implementation.solve(&mut problem);

    // Save solution
    solution
        .to_file(&args.output_dir)
        .expect("Failed to save solution to file");

    println!("Solution saved to {:?}", args.output_dir);
}

pub fn run_generator<G>(generator_implementation: G)
where
    G: ProblemGenerator,
{
    let args = GeneratorArgs::parse();
    println!(
        "Running generator with size: {}, seed: {}, output: {:?}",
        args.size,
        Some(args.seed.unwrap_or(0)).map_or("None".to_string(), |s| s.to_string()),
        args.output_dir
    );
    let instance = generator_implementation.generate(args.size, args.seed.unwrap_or(0));
    instance
        .to_file(&args.output_dir)
        .expect("Failed to save generated instance to file");
    println!("Generated instance saved to {:?}", args.output_dir);
}

pub fn run_verifier<V>(verifier_implementation: V)
where
    V: Verifier,
{
    let args = VerifierArgs::parse();
    println!(
        "Running verifier with input: {:?}, output: {:?}",
        args.input, args.output
    );

    // Load problem
    let problem = V::Problem::from_file(&args.input).expect("Failed to load problem from file");

    // Load solution
    let solution = V::Solution::from_file(&args.output).expect("Failed to load solution from file");

    // Verify solution
    let is_valid = verifier_implementation.verify(&problem, &solution);

    if is_valid {
        println!("The solution is valid.");
    } else {
        println!("The solution is invalid.");
    }
}
