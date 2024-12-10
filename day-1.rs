use std::error::Error;
use std::time::Instant;
use std::collections::HashMap;

fn parse_into_arrays(input: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut left_values = Vec::new();
    let mut right_values = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Invalid input format: Each line should contain exactly two integers.".into());
        }

        let left_num: i32 = parts[0].parse()?;
        let right_num: i32 = parts[1].parse()?;

        left_values.push(left_num);
        right_values.push(right_num);
    }

    Ok((left_values, right_values))
}

fn calculate_total_distance(left: &[i32], right: &[i32]) -> i32 {
    let mut left_sorted = left.to_vec();
    let mut right_sorted = right.to_vec();

    left_sorted.sort();
    right_sorted.sort();

    left_sorted.iter().zip(right_sorted.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn calculate_total_similarity(left: &[i32], right: &[i32]) -> i32 {
    // Count occurrences of each number in right_list for quick lookups
    let mut right_counts = HashMap::new();
    for &num in right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity score
    let mut similarity_score = 0;
    for &num in left {
        // If the number appears N times in right_list, increase score by num * N.
        if let Some(&count) = right_counts.get(&num) {
            similarity_score += num * count;
        }
    }

    // Return the similarity score
    similarity_score
}

fn main() -> Result<(), Box<dyn Error>> {
    // Using empty input here for demonstration, but you may want to replace
    // with actual input data.
    let input = ""; 

    // Time the parsing
    let parse_start = Instant::now();
    let (left, right) = parse_into_arrays(input)?;
    let parse_duration = parse_start.elapsed();
    println!("Parsing took: {:?}", parse_duration);

    // Time the distance calculation
    let calc_start = Instant::now();
    let total_distance = calculate_total_distance(&left, &right);
    let calc_duration = calc_start.elapsed();
    println!("Distance calculation took: {:?}", calc_duration);
    println!("Total distance: {}", total_distance);
    
    // Time the similarity calculation
    let calc_two_start = Instant::now();
    let total_similarity = calculate_total_similarity(&left, &right);
    let calc_two_duration = calc_two_start.elapsed();
    println!("Similarity calculation took: {:?}", calc_two_duration);
    println!("Total similarity: {}", total_similarity);

    Ok(())
}
