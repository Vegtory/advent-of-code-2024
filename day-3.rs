use std::error::Error;
use std::time::Instant;
use regex::Regex;

fn sum_enabled_mul(input: &str) -> Result<i32, Box<dyn Error>> {
let re = Regex::new(
        r"(?P<mul>mul\(([0-9]{1,3}),([0-9]{1,3})\))|(?P<do>do\(\))|(?P<dont>don't\(\))"
    )?;

    let mut total = 0;
    let mut enabled = true; // Initially, mul instructions are enabled

    for cap in re.captures_iter(input) {
        if cap.name("mul").is_some() {
            // Extract the two numbers. Groups 2 and 3 in the regex match the digits for X and Y.
            let x: i32 = cap[2].parse()?;
            let y: i32 = cap[3].parse()?;
            if enabled {
                total += x * y;
            }
        } else if cap.name("do").is_some() {
            enabled = true;
        } else if cap.name("dont").is_some() {
            enabled = false;
        }
    }

    Ok(total)
}

fn sum_valid_mul(input: &str) -> Result<i32, Box<dyn Error>> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;
    let mut total = 0;

    for cap in re.captures_iter(input) {
        let x: i32 = cap[1].parse()?;
        let y: i32 = cap[2].parse()?;
        total += x * y;
    }

    Ok(total)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = "\
";

    let start = Instant::now();
    let total = sum_enabled_mul(input)?;
    let duration = start.elapsed();

    println!("Total sum of all valid mul instructions: {}", total);
    println!("Time taken: {:?}", duration);

    Ok(())
}

// PART 1 Time taken: 296.467µs
// PART 2 Time taken: 683.764µs
