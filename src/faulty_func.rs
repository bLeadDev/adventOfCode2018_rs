#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_variables)]

use std::fs::{File, self};
use std::{io::{self, Read, Error}, str::Lines, error};


fn read_file_line_by_line_2(file_name: &str) -> Result<Vec<i32>, Error>{
    fs::read_to_string(file_name)?
    //.expect("Unexpected Error reading string")
    .lines()
    .into_iter()
    .map(|f| {
        f.parse::<i32>()?
        //.expect("Unexpected input. Probably not an int")
    })
    .collect()
}


fn main() {
    println!("Starting rust programm!");

    let lines = read_file_line_by_line_2("C:\\bLeadDev\\adventOfCode2018_rs\\advent_of_code\\src\\input.txt");
    println!("At Line 18!");


    println!("At Line 27!");

    println!("Hello, world!");
}
