use std::{env, fs::read_to_string};

fn main() {
    let Some(input) = env::args().nth(1) else {
        panic!("missing input");
    };
    let Ok(file) = read_to_string(&input) else {
        panic!("cannot open file");
    };
    let mut total: u32 = 0;

    for line in file.lines() {
        let line = &line[5..]; // remove prefix
        let Some((id, line)) = line.split_once(": ") else {
            panic!("error on split line")
        };

        if is_line_valid(line.to_string()) {
            let Ok(id) = id.parse::<u32>() else {
                panic!("failing parsing id '{}'", id);
            };
            total += id;
            println!("possible game => id: {} total: {}", id, total);
        }
    }

    println!("total: {}", total);
}

fn is_line_valid(line: String) -> bool {
    for round in line.split("; ") {
        for hand in round.split(", ") {
            if !is_possible_hard(hand.to_string(), (12, 13, 14)) {
                return false;
            }
        }
    }
    return true;
}

fn is_possible_hard(hand: String, (red, green, blue): (u32, u32, u32)) -> bool {
    let Some((num, color)) = hand.split_once(" ") else {
        panic!("error on split hand");
    };
    let Ok(num)  = num.parse::<u32>() else {
        panic!("failing parsing hand count {}:{}", num, color);
    };

    match color {
        "red" => num <= red,
        "green" => num <= green,
        "blue" => num <= blue,
        _ => {
            panic!("wrong color {}", color);
        }
    }
}
