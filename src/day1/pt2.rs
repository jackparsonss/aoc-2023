use regex::Regex;
use std::io;

const PATTERN: &str = r#"(?i)(\d|one|two|three|four|five|six|seven|eight|nine)"#;

fn main() {
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);

    let mut sum = 0;
    reader.into_inner().lines().for_each(|line| {
        if let Ok(line) = line {
            let s = get_str(&line);

            let parse: Result<u32, _> = s.parse();
            if let Ok(total) = parse {
                sum += total;
            }
        }
    });

    println!("Sum: {}", sum);
}

fn get_str(s: &str) -> String {
    let pattern = Regex::new(PATTERN).unwrap();

    let mut first = "";
    let mut last = "";
    for pattern in pattern.captures_iter(s) {
        if first.is_empty() {
            first = to_number(pattern.get(1).unwrap().as_str());
        } else {
            last = to_number(pattern.get(1).unwrap().as_str());
        }
    }

    if last.is_empty() {
        last = first;
    }

    first.to_string() + last
}

fn to_number(s: &str) -> &str {
    match s.to_lowercase().as_str() {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => s,
    }
}
