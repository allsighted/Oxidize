/*  Median and Mode

    How:
     Use a struct `Statistics` with 4 attributes:
         1. sorted vector of values
         2. median value
         3. hash-map that keeps track of value occurrences
         4. mode value
*/

use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
struct Statistics {
    ordered_values: Vec<i32>,         // sorted vector of values
    median: i32,              // median value
    occurrences: HashMap<i32, u32>,  // hash-map for value occurrences
    mode: i32,                // mode value
}

fn main() {
    // 1. take a vector of values and if it's an integer vector then convert it to f64 type first
    let int_unord_vec: Vec<i32> = vec![32, 132, 534, 123, 555, 534];
    
    // 2. order the vector and count the occurrences of the values
    let (int_ord_vec, occurances_hashmap) = order_and_count(int_unord_vec);

    // println!("{int_ord_vec:?} | {occurances_hashmap:?}");

    // 3. extract the median and mode
    let (median, mode) = extract_median_and_mode(&int_ord_vec, &occurances_hashmap);

    let statistics: Statistics = Statistics {
        ordered_values: int_ord_vec, 
        median: median, 
        occurrences: occurances_hashmap, 
        mode: mode 
    };

    println!("{statistics:#?}")
}

fn order_and_count(unord_vec: Vec<i32>) -> (Vec<i32>, HashMap<i32, u32>) {
    let mut sorted_vec = unord_vec.clone();
    sorted_vec.sort();

    // Create a HashMap to count occurrences
    let mut occurrences: HashMap<i32, u32> = HashMap::new();
    
    // Count each value's occurrences
    for &val in &sorted_vec {
        *occurrences.entry(val).or_insert(0) += 1;
    }

    (sorted_vec, occurrences)
}

fn extract_median_and_mode(ord_vec: &Vec<i32>, occurrences: &HashMap<i32, u32>) -> (i32, i32) {
    // Calculate median based on vector length (odd vs even)
    let len = ord_vec.len();
    let median = if len % 2 == 0 {
        // Even length - average of two middle elements
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (ord_vec[mid_left] + ord_vec[mid_right]) / 2
    } else {
        // Odd length - middle element
        ord_vec[len / 2]
    };

    // Find mode using max_by_key
    let mode = match occurrences.iter().max_by_key(|&(_, count)| count) {
        Some((&value, _)) => value,
        None => 0  // Default value if HashMap is empty
    };
    
    // Return tuple of (median, mode)
    (median, mode)
}
