// 2023 day 1

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();
    let mut sum: u32 = 0;

    for line in lines {
        sum += calibration_value(line);
    }

    println!("part 1 sum: {}", sum);
}

fn calibration_value(s: &str) -> u32 {
    let chars: Vec<char> = s.chars().collect();
    let first_digit: u32;
    let last_digit: u32;
    let mut digits: Vec<u32> = vec![];

    for c in chars {
        if c.is_numeric() {
            digits.push(c.to_digit(10).unwrap());
        }
    }

    match digits.len() {
        0 => {
            first_digit = 0;
            last_digit = 0;
        }
        1 => {
            first_digit = digits[0];
            last_digit = digits[0];
        }
        _ => {
            first_digit = digits[0];
            last_digit = digits[digits.len() - 1];
        }
    };

    first_digit * 10 + last_digit
}

fn part2() {
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();
    let mut sum: u32 = 0;

    for line in lines {
        let first = find_first_digit(line.to_string());
        let last = find_last_digit(line.to_string());
        sum += first * 10 + last;
    }
    println!("part 2 sum: {}", sum);
}

fn find_first_digit(line: String) -> u32 {
    let digit_strings = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut val: u32 = 0;
    let mut flag = false;

    for i in 0..line.len() {
        let slice = &line[i..];
        for d in digit_strings {
            if slice.starts_with(d) {
                val = text_to_digit(d);
                flag = true;
                break;
            }
        }
        let ch = slice.chars().next().unwrap();
        if ch.is_numeric() {
            val = ch.to_string().parse::<u32>().unwrap();
            flag = true;
        }
        if flag {
            break;
        }
    }

    val
}

fn find_last_digit(line: String) -> u32 {
    let digit_strings = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut val: u32 = 0;
    let mut flag = false;

    for i in (0..line.len()).rev() {
        let slice = &line[i..];
        for d in digit_strings {
            if slice.starts_with(d) {
                val = text_to_digit(d);
                flag = true;
                break;
            }
        }
        let ch = slice.chars().next().unwrap();
        if ch.is_numeric() {
            val = ch.to_string().parse::<u32>().unwrap();
            flag = true;
        }
        if flag {
            break;
        }
    }

    val
}

fn text_to_digit(s: &str) -> u32 {
    match s {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}
