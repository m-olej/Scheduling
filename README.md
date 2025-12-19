# Theory of scheduling problems

**problem_1** - `1 | r_j, S_ij | Sum C_j`

**problem_2** - `Q5 | r_j | Sum_Yj`

## Usage

**scripts**:
* generate_set.sh `output_directory` `generator_executable`
    * generates instances of size [50, 500] with step 50 to output_directory
* verify_set.sh `dataset_directory` `verifier_executable`
    * verifier sets of instance & solution files
* solver_set.sh `dataset_directory` `output_directory` `solver_executable`
    * solves sets of instances using passed solver executable and writes solutions to one output_dir

**dataset_dir** format:
```txt
  * dataset_directory structure
  ```txt
    └── dataset_dir/
        ├── set1/
        │   ├── in/
        │   │   ├── instance1
        │   │   └── ...
        │   └── out/
        │       ├── solution1
        │       └── ...
        └── ...
```

**build-specific**:
* cargo build --release --bin `bin_name`
  * generated executable at target/release/`bin_name`

**debug**
Use vscodes run and debug tool to debug a specified program using specified args:
* binary prefix eg..: p2
* binary type eg..: solver
* program args file eg..: solver.args

***.args** format:
```txt
--named-var
named-var-value
optional-var-value
```
## Structure

```txt
├── .vscode/
│   ├── launch.json                 <- debug runner
│   ├── tasks.json              
│   ├── generator.args             <- debugger args files:
│   ├── verifier.args                   -||-
│   └── solver.args                     -||-
├── scripts/
│   ├── generate_set.sh
│   ├── solve_set.sh
│   └── verify_set.sh
└── src/
    ├── bin/
    │   ├── px_generator          
    │   ├── px_verifier
    │   └── px_solver
    ├── problem_x/
    │   ├── problem.md              <- Problem definition with technical details
    │   ├── mod.rs                      <- export module
    │   ├── models.rs                  <- problem specific models definition [implements lib traits]
    │   ├── generate.rs                <- generator implementation
    │   ├── verify.rs                    <- verifier implementation
    │   ├── solve.rs                     <- solver implementation
    │   └── algo_x/                     <- Specific algorythm components/
    │       ├── algo_component1.rs
    │       └── algo_component2.rs
    └── lib.rs
```

## Library implementation

### Generic data models
```rust
/// A generic result type
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
```

### Generic traits

```rust
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
```
### CLI arguments 

```rust
/// Solver program for scheduling problems
///
/// This function runs a solver implementation that solves given problem instances
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

```

### Problem binary format

```rust
/// Generator
use scheduling::problem_x::generate::Generator;
use scheduling::run_generator;

fn main() {
    env_logger::init();
    let generator = Generator {};
    run_generator(generator);
}

/// Verifier
use scheduling::problem_x::verify::Verifier;
use scheduling::run_verifier;

fn main() {
    env_logger::init();
    let verifier = Verifier {};
    run_verifier(verifier);
}

/// Solver
use scheduling::problem_x::solve::Solver;
use scheduling::run_solver;

fn main() {
    env_logger::init();
    let solver = Solver {};
    run_solver(solver);
}
```

