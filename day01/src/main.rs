use once_cell::sync::Lazy;
use regex::{Captures, Regex};


const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Day 01 part 1: {}", get_sum_of_calibration_values(INPUT));

    println!("Day 01 part 2: {}", get_spelled_sum_of_calibration_values(INPUT));
}

fn get_calibration_value(line: impl AsRef<str>) -> u32 {
    let last_digit = line
        .as_ref()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .last()
        .expect("No digit found");
    let first_digit = line
        .as_ref()
        .chars()
        .find_map(|c| c.to_digit(10))
        .expect("No digit found");
    first_digit * 10 + last_digit
}

fn get_spelled_sum_of_calibration_values(input: &str) -> u32 {
    input.lines().map(ReplaceSpelledNumbers::replace_spelled_numbers).map(get_calibration_value).sum()
}

fn get_sum_of_calibration_values(input: &str) -> u32 {
    input.lines().map(get_calibration_value).sum()
}

trait ReplaceSpelledNumbers {
    fn replace_spelled_numbers(self) -> String;
}

impl ReplaceSpelledNumbers for &str {
    fn replace_spelled_numbers(self) -> String {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap());
        let s = RE.replace_all(self, |cap: &Captures| {
            let v = match &cap[1] {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => unreachable!()
            };
            format!("{0}{1}", v, &cap[1][1..])
        }).to_string();
        RE.replace_all(&s, |cap: &Captures| {
            let v = match &cap[1] {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => unreachable!()
            };
            format!("{0}{1}", v, &cap[1][1..])
        }).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE1: &str = include_str!("example1.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");

    #[test]
    fn it_gets_calibration_value() {
        let mut lines = EXAMPLE1.lines();
        assert_eq!(12, get_calibration_value(lines.next().unwrap_or_default()));
        assert_eq!(38, get_calibration_value(lines.next().unwrap_or_default()));
        assert_eq!(15, get_calibration_value(lines.next().unwrap_or_default()));
        assert_eq!(77, get_calibration_value(lines.next().unwrap_or_default()));
    }

    #[test]
    fn it_gets_calibration_value_with_spelled_numbers() {
        let mut lines = EXAMPLE2.lines();
        assert_eq!(29, get_calibration_value(&lines.next().unwrap_or_default().replace_spelled_numbers()));
        assert_eq!(83, get_calibration_value(&lines.next().unwrap_or_default().replace_spelled_numbers()));
        assert_eq!(13, get_calibration_value(&lines.next().unwrap_or_default().replace_spelled_numbers()));
        assert_eq!(24, get_calibration_value(&lines.next().unwrap_or_default().replace_spelled_numbers()));
        assert_eq!(42, get_calibration_value(&lines.next().unwrap_or_default().replace_spelled_numbers()));
        assert_eq!(14, get_calibration_value(&lines.next().unwrap_or_default().replace_spelled_numbers()));
        assert_eq!(76, get_calibration_value(&lines.next().unwrap_or_default().replace_spelled_numbers()));
    }

    #[test]
    fn it_gets_sum_with_spelled_numbers() {
        assert_eq!(281, get_spelled_sum_of_calibration_values(EXAMPLE2));
    }

    #[test]
    fn it_gets_very_short_numbers() {
        assert_eq!(77, get_calibration_value("v7"));
    }
    #[test]
    fn it_gets_overlapping_numbers() {
        assert_eq!(38, get_calibration_value("threeight".replace_spelled_numbers()));
    }
}
