use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub ready_time: u32,
    pub processing_time: u32,
    pub switch_time: Vec<u32>,
}

pub struct Instance {
    pub n: usize,
    pub tasks: Vec<Task>,
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Instance(n: {})", self.n)?;
        for task in &self.tasks {
            writeln!(
                f,
                "  Task(id: {}, p_j: {}, r_j: {}, s_ij: {:?})",
                task.id, task.processing_time, task.ready_time, task.switch_time
            )?;
        }
        Ok(())
    }
}

impl Instance {
    pub fn read(file_content: &String) -> Self {
        let mut lines = file_content.lines();

        let n = lines.next().unwrap_or("0").parse().unwrap_or(0);
        println!("Number of tasks (n): {}", n);

        let mut tasks = Vec::with_capacity(n);
        for (id, line) in lines.enumerate() {
            if id as u32 <= n as u32 - 1 {
                let mut parts = line.split_whitespace();
                let processing_time = parts.next().unwrap_or("0").parse().unwrap_or(0);
                let ready_time = parts.next().unwrap_or("0").parse().unwrap_or(0);
                let switch_time = vec![0; n];

                tasks.push(Task {
                    id: id as u32,
                    processing_time,
                    ready_time,
                    switch_time,
                });
            } else {
                let task_id = id - n;
                let mut parts = line.split_whitespace();
                for j in 0..n {
                    let setup_time = parts.next().unwrap_or("0").parse().unwrap_or(0);
                    tasks[task_id].switch_time[j] = setup_time;
                }
            }
        }

        return Self { n: n, tasks: tasks };
    }

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
                let setup_time = self.tasks[i].switch_time[j];
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

pub struct Solution<'a> {
    duration: u128,
    score: u32,
    pub tasks: Vec<&'a Task>, // (start_time, task_ref)
}

impl<'a> Solution<'a> {
    pub fn new(tasks: Vec<&'a Task>) -> Self {
        Self {
            duration: 0,
            score: 0,
            tasks: tasks,
        }
    }

    pub fn set_score(&mut self, score: u32) {
        self.score = score;
    }

    pub fn set_duration(&mut self, duration: u128) {
        self.duration = duration;
    }

    pub fn format(&self) -> String {
        let mut output = String::new();
        output += format!("{}\n", self.score).as_str();
        for task in &self.tasks {
            output += format!("{} ", task.id).as_str();
        }

        output
    }
}
