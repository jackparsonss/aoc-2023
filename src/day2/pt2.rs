use std::collections::HashMap;
use std::io;

fn main() {
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);

    let mut sum = 0;
    reader.into_inner().lines().for_each(|line| {
        if let Ok(line) = line {
            let mut map: HashMap<&str, i32> = HashMap::new();
            map.insert("blue", 0);
            map.insert("red", 0);
            map.insert("green", 0);

            // lhs is "game #", rhs is game
            let split: Vec<&str> = line.split(':').collect();

            let game = split[1];
            let rounds: Vec<&str> = game.split(';').collect();
            for round in rounds {
                let colors: Vec<&str> = round.split(',').collect();

                for color in colors {
                    let split: Vec<&str> = color.split(' ').collect();
                    let num = split[1];
                    let color = split[2];

                    let num: Result<i32, _> = num.parse();
                    if let Ok(num) = num {
                        if let Some(val) = map.get(color) {
                            if num > *val {
                                map.insert(color, num);
                            }
                        }
                    }
                }
            }

            let mut temp = 1;
            for value in map.values() {
                temp *= value;
            }

            sum += temp;
        }
    });

    println!("Sum: {}", sum);
}
