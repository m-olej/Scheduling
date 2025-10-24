use crate::models::{Instance, Schedule};

pub fn verify_instance(content: &str) -> Result<(), String> {
    let mut lines = content.lines();

    // --- Check 1: Parse n (number of tasks) ---
    let n_line = lines.next().ok_or("File is empty. Cannot read 'n'.")?;
    let n: usize = n_line.trim().parse().map_err(|_| {
        format!(
            "Failed to parse the number of tasks 'n' from line: '{}'",
            n_line
        )
    })?;

    if n == 0 {
        return Err("Number of tasks 'n' cannot be zero.".to_string());
    }

    // --- Check 2: Validate the total number of lines ---
    // We need 1 (for n) + n (for p_j, r_j) + n (for S_ij matrix rows)
    let expected_line_count = 1 + 2 * n;
    // We already consumed one line, so add it back for the total count.
    if lines.clone().count() + 1 != expected_line_count {
        return Err(format!(
            "Incorrect number of lines. Expected {}, but file has {}.",
            expected_line_count,
            content.lines().count()
        ));
    }

    // --- Check 3: Parse and validate p_j and r_j ---
    for i in 0..n {
        let line_num = i + 2;
        let line = lines.next().unwrap(); // Safe due to previous line count check
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != 2 {
            return Err(format!(
                "Line {}: Expected 2 values (p_j, r_j), but found {}.",
                line_num,
                parts.len()
            ));
        }
        let p_j: u32 = parts[0].parse().map_err(|_| {
            format!(
                "Line {}: Failed to parse processing time '{}'.",
                line_num, parts[0]
            )
        })?;
        if p_j == 0 {
            return Err(format!(
                "Line {}: Processing time p_j must be positive.",
                line_num
            ));
        }
        // Check r_j
        let _: u32 = parts[1].parse().map_err(|_| {
            format!(
                "Line {}: Failed to parse ready time '{}'.",
                line_num, parts[1]
            )
        })?;
    }

    // --- Check 4: Parse and validate the S_ij matrix ---
    for i in 0..n {
        let line_num = i + 2 + n;
        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != n {
            return Err(format!(
                "Line {}: Expected {} setup time values for S_ij row, but found {}.",
                line_num,
                n,
                parts.len()
            ));
        }

        for (j, val_str) in parts.iter().enumerate() {
            let s_ij: u32 = val_str.parse().map_err(|_| {
                format!(
                    "Line {}: Failed to parse setup time at column {}.",
                    line_num,
                    j + 1
                )
            })?;

            // The most important constraint for S_ij!
            if i == j && s_ij != 0 {
                return Err(format!(
                    "Constraint violation on line {}: Diagonal setup time S_{}{
                    } must be 0, but was {}.",
                    line_num, i, i, s_ij
                ));
            }
        }
    }

    Ok(())
}

pub fn calculate_score(schedule: Schedule, instance: Instance) -> String {
    let mut score = 0;
    let mut output = String::new();
    let mut task_string = String::new();

    for (start_time, task_id) in schedule.tasks {
        score += start_time + instance.tasks[task_id as usize].processing_time;
        task_string += &format!("{} ", task_id);
    }

    output += &format!("{}\n", score);
    output += &task_string;

    output
}
