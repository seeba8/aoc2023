use std::collections::HashMap;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let numbers = parse_input_for_parts(INPUT);
    println!("Day 03 part 1: {}", numbers.iter().filter_map(|n| n.symbol.and(Some(n.value))).sum::<usize>());
    let gears = parse_input_for_gear_ratios(INPUT);
    println!("Day 03 part 2: {}", gears.iter().sum::<usize>());
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Number {
    value: usize,
    symbol: Option<char>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Number2 {
    value: usize,
    start_index: usize,
}


fn parse_input_for_gear_ratios(input: &str) -> Vec<usize> {
    let line_length = input.lines().next().unwrap().len();
    let x = input.trim().replace(['\r', '\n'], "");
    let mut gears: Vec<usize> = Vec::new();
    for (i, _) in x.as_bytes().iter().enumerate().filter(|(_, c)| **c == b'*') {
        let numbers = check_neighbours_for_numbers(x.as_bytes(), line_length, i);
        if numbers.len() == 2 {
            gears.push(numbers[0] * numbers[1]);
        }
    }
    gears
}

fn parse_input_for_parts(input: &str) -> Vec<Number> {
    let line_length = input.lines().next().unwrap().len();
    let x = input.trim().replace(['\r', '\n'], "");
    let mut numbers: Vec<Number> = Vec::new();
    let mut current_number: Option<Number> = None;
    for (i, c) in x.as_bytes().iter().enumerate() {
        if i % line_length == 0 {
            if let Some(n) = current_number {
                numbers.push(n);
                current_number = None;
            }
        }
        match c {
            c if c.is_ascii_digit() => {
                match &mut current_number {
                    None => {
                        current_number = Some(Number {
                            value: (c - 0x30) as usize,
                            symbol: check_neighbours_for_symbol(x.as_bytes(), line_length, i),
                        });
                    }
                    Some(n) => {
                        n.value = n.value * 10 + (c - 0x30) as usize;
                        if n.symbol.is_none() {
                            n.symbol = check_neighbours_for_symbol(x.as_bytes(), line_length, i);
                        }
                    }
                }
            }
            _ => {
                if let Some(n) = current_number {
                    numbers.push(n);
                    current_number = None;
                }
            }
        }
    }
    if let Some(n) = current_number {
        numbers.push(n);
    }
    numbers
}

fn check_neighbours_for_symbol(slice: &[u8], line_length: usize, index: usize) -> Option<char> {
    let x = index % line_length;
    let y = index / line_length;
    let height = slice.len() / line_length;
    for (x_offset, y_offset) in (-1isize..=1).cartesian_product(-1isize..=1) {
        if x_offset == 0 && y_offset == 0 {
            continue;
        }
        if x.checked_add_signed(x_offset).is_none()
            || x.saturating_add_signed(x_offset) >= line_length
            || y.checked_add_signed(y_offset).is_none()
            || y.saturating_add_signed(y_offset) >= height {
            continue;
        }
        let neighbour = slice[y.saturating_add_signed(y_offset) * line_length + x.saturating_add_signed(x_offset)];
        match neighbour {
            b'.' => {}
            x if x.is_ascii_digit() => {}
            x => { return Some(char::from(x)); }
        }
    }
    None
}

fn check_neighbours_for_numbers(slice: &[u8], line_length: usize, index: usize) -> Vec<usize> {
    let x = index % line_length;
    let y = index / line_length;
    let height = slice.len() / line_length;
    let mut neighbours = HashMap::new();
    for (x_offset, y_offset) in (-1isize..=1).cartesian_product(-1isize..=1) {
        if x_offset == 0 && y_offset == 0 {
            continue;
        }
        if x.checked_add_signed(x_offset).is_none()
            || x.saturating_add_signed(x_offset) >= line_length
            || y.checked_add_signed(y_offset).is_none()
            || y.saturating_add_signed(y_offset) >= height {
            continue;
        }
        let neighbour_index = y.saturating_add_signed(y_offset) * line_length + x.saturating_add_signed(x_offset);
        let neighbour = slice[neighbour_index];
        if neighbour.is_ascii_digit() {
            let complete_number = get_complete_number(slice, line_length, neighbour_index);
            neighbours.insert(complete_number.start_index, complete_number.value);
        }
    }
    neighbours.values().copied().collect_vec()
}

const fn get_complete_number(slice: &[u8], line_length: usize, index: usize) -> Number2 {
    let current_line = index / line_length;
    let mut x = index % line_length;
    let mut did_step = false;
    while x > 0 && slice[current_line * line_length + x].is_ascii_digit() {
        x -= 1;
        did_step = true;
    }
    if did_step && !slice[current_line * line_length + x].is_ascii_digit() {
        x += 1;
    }
    let mut index = current_line * line_length + x;
    let start_index = index;
    let mut result = 0;
    while index / line_length == current_line && slice[index].is_ascii_digit() {
        result = 10 * result + (slice[index] - 0x30) as usize;
        index += 1;
    }
    Number2 { value: result, start_index }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("example.txt");

    #[test]
    fn it_gets_numbers() {
        let input = r"467..114..
...*......";
        let numbers = parse_input_for_parts(input);
        let expected = vec![Number { value: 467, symbol: Some('*') }, Number { value: 114, symbol: None }];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn it_gets_numbers_with_symbols() {
        let numbers = parse_input_for_parts(EXAMPLE1);
        assert_eq!(4361_usize, numbers.iter().filter_map(|n| n.symbol.and(Some(n.value))).sum());
    }

    #[test]
    fn it_gets_numbers_separated_by_symbol() {
        let input = r"467#114";
        let numbers = parse_input_for_parts(input);
        let expected = vec![Number { value: 467, symbol: Some('#') }, Number { value: 114, symbol: Some('#') }];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn it_gets_single_number() {
        let input = r"123";
        let numbers = parse_input_for_parts(input);
        let expected = vec![Number { value: 123, symbol: None }];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn it_gets_single_number_with_symbol() {
        let input = r"123#";
        let numbers = parse_input_for_parts(input);
        let expected = vec![Number { value: 123, symbol: Some('#') }];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn it_gets_single_number_with_preceding_symbol() {
        let input = r"#123";
        let numbers = parse_input_for_parts(input);
        let expected = vec![Number { value: 123, symbol: Some('#') }];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn it_gets_asymmetric_file() {
        let input = r"123
...";
        let numbers = parse_input_for_parts(input);
        let expected = vec![Number { value: 123, symbol: None }];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn it_gets_asymmetric_file_with_symbol() {
        let input = r"..#
123";
        let numbers = parse_input_for_parts(input);
        let expected = vec![Number { value: 123, symbol: Some('#') }];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn it_gets_separate_numbers() {
        let input = r"123
12#";
        let numbers = parse_input_for_parts(input);
        let expected = vec![Number { value: 123, symbol: Some('#') }, Number { value: 12, symbol: Some('#') }];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn it_gets_complete_number() {
        let input = r"..12345..";
        assert_eq!(get_complete_number(input.as_bytes(), 3, 2), Number2 { value: 1, start_index: 2 });
        assert_eq!(get_complete_number(input.as_bytes(), 3, 3), Number2 { value: 234, start_index: 3 });
        assert_eq!(get_complete_number(input.as_bytes(), 3, 4), Number2 { value: 234, start_index: 3 });
        assert_eq!(get_complete_number(input.as_bytes(), 3, 5), Number2 { value: 234, start_index: 3 });
        assert_eq!(get_complete_number(input.as_bytes(), 3, 6), Number2 { value: 5, start_index: 6 });
    }

    #[test]
    fn it_finds_gear_rations() {
        let gears = parse_input_for_gear_ratios(EXAMPLE1);
        assert_eq!(gears.iter().sum::<usize>(), 467_835);
    }
}