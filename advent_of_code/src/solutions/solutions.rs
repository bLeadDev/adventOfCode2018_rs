use std::collections::HashSet;

pub mod utils;
use crate::utils::string_reader;

pub fn day01() {
    /* DAY 01 */
    let lines = read_file_to_vec("/workspaces/adventOfCode2018_rs/advent_of_code/src/input.txt");
    let sum: i32 = lines.iter().sum();
    println!("Frequency of task 1 is: {sum}");

    let mut seen_frequencies: HashSet<i32> = HashSet::new();
    let mut actual_frequency = 0;
    'duplicate_found: loop{
        for frequency  in &lines{
            if seen_frequencies.insert(actual_frequency) == false {
                break 'duplicate_found;
            }else {
                actual_frequency = actual_frequency + frequency;
            }
        }
    }
    println!("Frequency of task 2 is: {actual_frequency}");
}