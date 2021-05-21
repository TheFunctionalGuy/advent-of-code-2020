use std::fs::File;
use std::io::{Error, BufReader, Read, BufRead, ErrorKind};

fn main() -> Result<(), Error> {
    // Read numbers from file
    let numbers = file_to_vector(File::open("src/bin/day_1/day_1.txt")?)?;

    let target = 2020;
    match two_numbers(&numbers, target) {
        None => println!("There are no two numbers that sum up to {}", target),
        Some((a, b)) => println!("The result of {} * {} is {}", a, b, a * b)
    };

    match three_numbers(&numbers, target) {
        None => println!("There are no three numbers that sum up to {}", target),
        Some((a, b, c)) => println!("The result of {} * {} * {} is {}", a, b, c, a * b * c)
    };

    Ok(())
}

// Get the product of two numbers that sum to target sum if there is any
fn two_numbers(numbers: &Vec<u32>, sum: u32) -> Option<(u32, u32)> {
    for n in 0..numbers.len() {
        for m in 0..numbers.len() {
            if numbers[n] + numbers[m] == sum {
                return Some((numbers[n], numbers[m]))
            }
        }
    }

    None
}

// Get the product of three numbers that sum to target sum if there is any
fn three_numbers(numbers: &Vec<u32>, sum: u32) -> Option<(u32, u32, u32)> {
    for n in 0..numbers.len() {
        for m in 0..numbers.len() {
            for o in 0..numbers.len() {
                if numbers[n] + numbers[m] + numbers[o] == sum {
                    return Some((numbers[n], numbers[m], numbers[o]))
                }
            }
        }
    }

    None
}

// Utility function used to convert a given file to a list of u32 valvues
fn file_to_vector<R: Read>(io: R) -> Result<Vec<u32>, Error> {
    let buffered_reader = BufReader::new(io);

    buffered_reader.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}