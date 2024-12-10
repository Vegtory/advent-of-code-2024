use std::error::Error;
use std::time::Instant;
use regex::Regex;

/// Finds all valid `mul(X,Y)` instructions in the given input string,
/// where X and Y are integers with 1–3 digits, and sums them up.
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
    // Example corrupted memory input
    let input = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

    let start = Instant::now();
    let total = sum_valid_mul(input)?;
    let duration = start.elapsed();

    println!("Total sum of all valid mul instructions: {}", total);
    println!("Time taken: {:?}", duration);

    Ok(())
}
