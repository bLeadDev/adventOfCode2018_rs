#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_variables)]

use std::fs::{File, self};
use core::fmt::Debug;
use std::{io::{self, Read, Error}, str::{Lines, FromStr}, error, ops::Index};
use std::collections::HashSet;
use std::collections::HashMap;

fn read_file_to_vec<T>(file_name: &str) -> Vec<T>
where //needed traits to work with the cheap .expect error "handling"
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
    //local path
    //let lines = read_file_to_vec("C:\\bLeadDev\\adventOfCode2018_rs\\advent_of_code\\src\\input_day1.txt");
    //codespace path
/*     
    let lines = read_file_to_vec("/workspaces/adventOfCode2018_rs/advent_of_code/src/input_day1.txt");
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
 */

    /* DAY02 */
    let lines: Vec<String> = read_file_to_vec("C:\\bLeadDev\\adventOfCode2018_rs\\src\\input_day2.txt");

    let mut chars_in_words:  Vec<HashMap<char, i32>> = vec![];
    for line in lines{
        let chars: HashMap<char, i32> = line
            .chars()
            .fold(HashMap::new(), |mut counts, c| 
            {
                *counts.entry(c).or_insert(0) += 1; // Count occurrences of each character
                counts
            });
        chars_in_words.push(chars);
    }

    //words where a duplet or triplet of characters only occure once 
    let mut count_of_three_letter_words  = 0;
    let mut count_of_two_letter_words    = 0;
    for word_char_count in chars_in_words{
        let values = word_char_count.values();
        let filtered = values.filter(|&x| *x == 3);
        if filtered.count() > 0{
            count_of_three_letter_words += 1;
        } 


        let values = word_char_count.values();
        //println!("values:\n{:#?}", values);
        let filtered = values.filter(|&x| *x == 2);
        //println!("filtered:\n{:#?}", filtered);
        if filtered.count() > 0{
            count_of_two_letter_words += 1;
        } 
        //println!("Count: {:#?}", cnt);
    }
    
    println!("Two letter words: {} Should be 244\nThree letter words: {} Should be 22", count_of_three_letter_words, count_of_two_letter_words);
    println!("Solution for Day02, Task1: {}", count_of_three_letter_words * count_of_two_letter_words);

}
