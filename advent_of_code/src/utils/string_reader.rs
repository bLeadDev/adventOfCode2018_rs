use std::fs::{self};
use core::fmt::Debug;
use std::str::FromStr;

pub fn read_file_to_vec<T>(file_name: &str) -> Vec<T>
where //needed traits to work with the cheap .expect error "handling2
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