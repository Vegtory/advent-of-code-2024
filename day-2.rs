use std::error::Error;
use std::time::Instant;

fn is_safe_report(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false; // Need at least two levels to determine increasing/decreasing
    }

    let first_diff = levels[1] - levels[0];
    if first_diff == 0 {
        // No change at the start means it can't be strictly monotonic
        return false;
    }

    let increasing = first_diff > 0;
    for window in levels.windows(2) {
        let diff = window[1] - window[0];
        if diff == 0 {
            // Adjacent levels cannot be equal
            return false;
        }

        // Check monotonic direction
        if (increasing && diff <= 0) || (!increasing && diff >= 0) {
            return false;
        }

        // Check difference magnitude
        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }
    }

    true
}

/// Checks if a report is safe as-is, or can be made safe by removing exactly one level.
/// Returns true if the report is safe without removal or if removing one level makes it safe.
fn is_safe_with_removal(levels: &[i32]) -> bool {
    // Check if already safe
    if is_safe_report(levels) {
        return true;
    }

    // Try removing each level once and check if safe
    for i in 0..levels.len() {
        let mut modified = levels.to_vec();
        modified.remove(i);
        if is_safe_report(&modified) {
            return true;
        }
    }

    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = "\
";

    let start = Instant::now();

    let mut safe_count = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue; // Skip empty lines
        }

        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if is_safe_with_removal(&levels) {
            safe_count += 1;
        }
    }

    let duration = start.elapsed();
    println!("Number of reports now safe (considering removal): {}", safe_count);
    println!("Time taken: {:?}", duration);

    Ok(())
}
