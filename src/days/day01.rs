use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let data = read_to_string("input/day01.txt").unwrap();
    let sol1: u32 = restore_calibration(&data);
    let sol2: u32 = restore_calibration_challenge(&data);

    (Solution::from(sol1), Solution::from(sol2))
}

// PART 1

pub fn restore_calibration(input: &str) -> u32 {
    input.lines().map(recover_calibration_value).sum()
}

pub fn recover_calibration_value(line: &str) -> u32 {
    let first_digit = find_number(line.chars());
    let last_digit = find_number(line.chars().rev());
    (first_digit * 10 + last_digit) as u32
}

pub fn find_number<I: Iterator<Item = char>>(chars: I) -> u8 {
    chars
        .filter(|c| c.is_numeric())
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap() as u8
}

// PART 2

pub fn restore_calibration_challenge(input: &str) -> u32 {
    input.lines().map(recover_calibration_challenge_value).sum()
}

pub fn recover_calibration_challenge_value(line: &str) -> u32 {
    let first_digit = find_number_challenge(line.chars(), false);
    let last_digit = find_number_challenge(line.chars().rev(), true);
    (first_digit * 10 + last_digit) as u32
}

pub fn find_number_challenge<I: Iterator<Item = char>>(mut chars: I, reverse: bool) -> u8 {
    let number_map = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut buffer = String::new();
    while let Some(c) = chars.next() {
        buffer.push(c);
        if c.is_numeric() {
            return c.to_digit(10).unwrap() as u8;
        } else {
            for (word, num) in &number_map {
                if reverse && buffer.ends_with(&word.chars().rev().collect::<String>()) {
                    return *num;
                } else if !reverse && buffer.ends_with(word) {
                    return *num;
                }
            }
        }
    }
    panic!("No number found in line: {}", buffer);
}

///////////////////////////////////////////////////////////////////////////////
/// Tests
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_CHALLENGE: &'static str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_restore_calibration() {
        assert_eq!(restore_calibration(EXAMPLE), 142);
    }

    #[test]
    fn test_restore_calibration_challenge() {
        assert_eq!(restore_calibration_challenge(EXAMPLE_CHALLENGE), 281);
    }
}
