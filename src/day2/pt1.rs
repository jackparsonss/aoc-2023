use std::collections::HashMap;
use std::io;

fn main() {
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);

    let mut max: HashMap<&str, i32> = HashMap::new();
    max.insert("blue", 14);
    max.insert("red", 12);
    max.insert("green", 13);

    let mut id_sum = 0;
    reader.into_inner().lines().for_each(|line| {
        if let Ok(line) = line {
            // lhs is "game #", rhs is game
            let mut is_valid = true;
            let split: Vec<&str> = line.split(':').collect();

            let game = split[1];
            let rounds: Vec<&str> = game.split(';').collect();
            'l: for round in rounds {
                let colors: Vec<&str> = round.split(',').collect();

                for color in colors {
                    let split: Vec<&str> = color.split(' ').collect();
                    let num = split[1];
                    let color = split[2];

                    let num: Result<i32, _> = num.parse();
                    if let Ok(num) = num {
                        if let Some(max) = max.get(color) {
                            if num > *max {
                                is_valid = false;
                                break 'l;
                            }
                        }
                    }
                }
            }

            // add game id if it's a valid game
            if is_valid {
                let id = split[0];
                let id: Vec<&str> = id.split(' ').collect();
                let id: Result<i32, _> = id[1].parse();
                if let Ok(id) = id {
                    id_sum += id;
                }
            }
        }
    });

    println!("Sum: {}", id_sum);
}
