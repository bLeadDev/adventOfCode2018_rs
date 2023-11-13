#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_variables)]

use std::fs::{File, self};
use core::fmt::Debug;
use std::{io::{self, Read, Error}, str::{Lines, FromStr}, error};
use std::collections::HashSet;

fn read_file_to_vec<T>(file_name: &str) -> Vec<T>
where //needed traits to work with the cheap .expect error handling
    T: std::str::FromStr,
    <T as FromStr>::Err: Debug,
{
    fs::read_to_string(file_name)
    .expect("Unexpected Error reading string")
    .lines()
    .into_iter()
    .map(|f| {
        f.parse::<T>()
        .expect("Unexpected input. Probably not the expected typ or misformated")
    })
    .collect()
}


fn main() {
    /* DAY 01 */
    let lines = read_file_to_vec("C:\\bLeadDev\\adventOfCode2018_rs\\advent_of_code\\src\\input.txt");
    let sum: i32 = lines.iter().sum();
    println!("Sum of day 1 is: {sum}");

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
    println!("Frequency of day 2 is: {actual_frequency}");
}
