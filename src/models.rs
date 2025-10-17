pub struct Task {
    id: u32,
    ready_time: u32,
    duration: u32,
    switch_time: Vec<(u32, u32)>, // (task_id, switch_time)
}

pub struct Schedule {
    tasks: Vec<Task>,
}
