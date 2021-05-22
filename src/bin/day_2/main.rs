#[macro_use]
extern crate lazy_static;

use helper::lines_from_file;
use regex::Regex;

fn main() {
    let password_and_policy_list = lines_from_file("src/bin/day_2/day_2.txt");
    let parsed_list: Vec<(PasswordPolicy, String)> = password_and_policy_list.iter()
        .filter_map(|entry| parse_list(entry))
        .collect();
    assert_eq!(parsed_list.len(), 1000);

    // Get number of valid passwords for the first interpretation of rules
    let num_valid_passwords = parsed_list.iter()
        .filter(|(policy, password)| is_valid_password(policy, password))
        .count();

    println!("Number of valid passwords: {}", num_valid_passwords);

    // Get number of valid password for the second interpretation of rules
    let num_valid_passwords_new_rules = parsed_list.iter()
        .filter(|(policy, password)| is_valid_password_new_rules(policy, password))
        .count();

    println!("Number of valid passwords according to the new interpretation of rules: {}", num_valid_passwords_new_rules);
}

// Struct that represents a password policy
struct PasswordPolicy {
    indices: (u32, u32),
    letter: char
}

// Parse the list of password polices and passwords
fn parse_list(entry: &str) -> Option<(PasswordPolicy, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<from>\d+)-(?P<to>\d+) (?P<letter>\w): (?P<password>\w+)").unwrap();
    }

    match RE.captures(&entry) {
        Some(caps) => {
            let first: u32 = caps["from"].parse::<u32>().unwrap();
            let second: u32 = caps["to"].parse::<u32>().unwrap();
            let letter: char = caps["letter"].chars().nth(0).unwrap();
            let password = caps["password"].to_string();

            Some((PasswordPolicy{ indices: (first, second), letter }, password))
        },
        None => None
    }
}

// Check if a password is set according to its policy
fn is_valid_password(password_policy: &PasswordPolicy, password: &String) -> bool {
    let mut letter_count = 0;

    for letter in password.chars() {
        if letter == password_policy.letter {
            letter_count += 1;
        }
    }

    // Occurrences of given letter have to be in the correct range
    if letter_count >= password_policy.indices.0 && letter_count <= password_policy.indices.1 {
        true
    } else {
        false
    }
}

// Check if password is set according to the new interpretation of the rules
fn is_valid_password_new_rules(password_policy: &PasswordPolicy, password: &String) -> bool {
    return (password.chars().nth(password_policy.indices.0 as usize - 1).unwrap() == password_policy.letter) ^
        (password.chars().nth(password_policy.indices.1 as usize - 1).unwrap() == password_policy.letter)
}
