use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = "\
";

    let grid: Vec<&str> = input.lines().collect();
    let word = "XMAS";
    let rows = grid.len();
    let cols = grid[0].len();

    let directions = [
        (-1, 0),  // up
        (1, 0),   // down
        (0, -1),  // left
        (0, 1),   // right
        (-1, -1), // up-left
        (-1, 1),  // up-right
        (1, -1),  // down-left
        (1, 1),   // down-right
    ];

    let mut count = 0;

    let start = std::time::Instant::now();

    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in &directions {
                if check_direction(&grid, r, c, dr, dc, word) {
                    count += 1;
                }
            }
        }
    }

    let duration = start.elapsed();
    println!("The word '{}' appears {} times.", word, count);
    println!("Time taken: {:?}", duration);

    Ok(())
}

/// Checks if `word` can be found starting at (row, col) in the direction (dr, dc).
fn check_direction(grid: &[&str], start_r: usize, start_c: usize, dr: isize, dc: isize, word: &str) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let length = word.len() as isize;

    let end_r = start_r as isize + (length - 1) * dr;
    let end_c = start_c as isize + (length - 1) * dc;

    if end_r < 0 || end_r >= rows || end_c < 0 || end_c >= cols {
        return false; // Out of bounds
    }

    for i in 0..length {
        let r = start_r as isize + i * dr;
        let c = start_c as isize + i * dc;
        if grid[r as usize].chars().nth(c as usize).unwrap() != word.chars().nth(i as usize).unwrap() {
            return false;
        }
    }

    true
}

// Time taken: 26.723436ms
