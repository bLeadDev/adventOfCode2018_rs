#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_variables)]

use std::fs::{File, self};
use core::fmt::Debug;
use std::{io::{self, Read, Error}, str::{Lines, FromStr}, error, ops::Index};
use std::collections::HashSet;
use std::collections::HashMap;

/// Returns a vector with the given type. 
/// Parsing or file read errors cause panic with a message.  
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


/// Returns if the two strings only differ only at one position 
fn cmp_str_only_one_differs_same_pos(str1: &str, str2: &str) -> bool{
    let mut one_error_found = false;
    for (ch1, ch2) in str1.chars().zip(str2.chars()) {
        if ch1 != ch2 && one_error_found == false{
            one_error_found = true;
        }else if ch1 != ch2 {
            return false;
        }
    }
    return true;
}

/// Returns a copy of the string without the first char that differs.
fn delete_differing_char(str1 :&str, str2: &str) -> Option<String>{
    for (idx, (ch1, ch2)) in str1.chars().zip(str2.chars()).enumerate() {
        if ch1 != ch2{
            let extracted_string = format!("{}{}", &str1[..idx], &str1[(idx + 1)..]);
            return Some(extracted_string);
        }
    }
    None
}

/// Returns the first duplicate frequency found when adding the vector over and over.
fn search_for_first_duplicate(lines: &Vec<i32>) -> i32{
    let mut seen_frequencies: HashSet<i32> = HashSet::new();
    let mut actual_frequency = 0;
    'duplicate_found: loop{
        for frequency  in lines{
            if seen_frequencies.insert(actual_frequency) == false {
                break 'duplicate_found;
            }else {
                actual_frequency = actual_frequency + frequency;
            }
        }
    }
    return actual_frequency
}

/// Returns a Hashmap of the string. Key is the char, value is the count of this char.
fn count_each_char(line: &str) -> HashMap<char, i32>{
    let chars: HashMap<char, i32> = line
    .chars()
    .fold(HashMap::new(), |mut counts, c| 
    {
        *counts.entry(c).or_insert(0) += 1; // Count occurrences of each character
        counts
    });
    chars
}


fn main() {
    /* DAY 01 TASK 1*/
    //local path
    let lines = read_file_to_vec("C:\\bLeadDev\\adventOfCode2018_rs\\src\\input_day1.txt");
    //codespace path
    //let lines = read_file_to_vec("/workspaces/adventOfCode2018_rs/advent_of_code/src/input_day1.txt");
    
    let sum: i32 = lines.iter().sum();
    println!("Frequency of task 1 is: {sum}");

    /* DAY 01 TASK 2*/
    let first_duplicate = search_for_first_duplicate(&lines);
    println!("Frequency of task 2 is: {first_duplicate}");
 

    /* DAY02 TASK 1*/
    let lines: Vec<String> = read_file_to_vec("C:\\bLeadDev\\adventOfCode2018_rs\\src\\input_day2.txt");

    let mut chars_in_words:  Vec<HashMap<char, i32>> = vec![];
    for line in &lines{
        let char_count = count_each_char(line);
        chars_in_words.push(char_count);
    }

    //words where a duplet or triplet of characters only occure once 
    let mut count_of_three_letter_words  = 0;
    let mut count_of_two_letter_words    = 0;
    for word_char_count in chars_in_words{
        if word_char_count.values()
            .filter(|&x| *x == 3)
            .count() > 0{
            count_of_three_letter_words += 1;
        } 
        if word_char_count.values()
            .filter(|&x| *x == 2)
            .count() > 0{
            count_of_two_letter_words += 1;
        } 
    }
    
    println!("Solution for Day02, Task1: {}", count_of_three_letter_words * count_of_two_letter_words);

    /* DAY02 TASK 2 */
    let lines: Vec<String> = read_file_to_vec("C:\\bLeadDev\\adventOfCode2018_rs\\src\\input_day2.txt");

    'outer_loop: for line in &lines{
        for inner_line in &lines{
            if inner_line != line && cmp_str_only_one_differs_same_pos(&line, &inner_line){
                let solution_day_2_2 = delete_differing_char(&line, &inner_line);
                println!("Solution Day02, Task 2: {}", solution_day_2_2.unwrap());
                break 'outer_loop;
            }
        }   
    }
}
