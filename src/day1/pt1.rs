use std::io;

fn main() {
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);

    let mut sum = 0;
    reader.into_inner().lines().for_each(|line| {
        if let Ok(line) = line {
            let total = get_first(&line).to_string() + &get_last(&line).to_string();

            let parse: Result<i32, _> = total.parse();
            if let Ok(total) = parse {
                sum += total;
            }
        }
    });

    println!("Sum: {}", sum);
}

fn get_first(s: &str) -> char {
    for c in s.chars() {
        if c.is_numeric() {
            return c;
        }
    }
    '\0'
}

fn get_last(s: &str) -> char {
    for c in s.chars().rev() {
        if c.is_numeric() {
            return c;
        }
    }
    '\0'
}
