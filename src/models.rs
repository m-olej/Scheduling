use std::collections::HashMap;
use std::fmt::Write;

pub struct Task {
    pub id: u32,
    pub ready_time: u32,
    pub processing_time: u32,
    pub switch_time: HashMap<u32, u32>, // (task_id, switch_time)
}
pub struct Instance {
    pub n: usize,
    pub tasks: Vec<Task>,
}

impl Instance {
    pub fn format(&self) -> String {
        if self.n == 0 {
            return "0\n".to_string();
        }

        let mut output = String::with_capacity(self.n * self.n * 4);

        // Instance size
        writeln!(&mut output, "{}", self.n).unwrap();

        // Tasks (p_j r_j)
        for task in &self.tasks {
            writeln!(&mut output, "{} {}", task.processing_time, task.ready_time).unwrap();
        }

        // Setup times (s_ij)
        for i in 0..self.n {
            for j in 0..self.n {
                let setup_time = *self.tasks[i].switch_time.get(&(j as u32)).unwrap();
                write!(
                    &mut output,
                    "{}{}",
                    setup_time,
                    if j == self.n - 1 { "\n" } else { " " }
                )
                .unwrap();
            }
        }

        output
    }
}

// pub struct Schedule {
//     tasks: Vec<(u32, Task)>, // (start_time, Task)
// }
