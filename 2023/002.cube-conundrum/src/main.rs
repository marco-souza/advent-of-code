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
        let Ok(id) = id.parse::<u32>() else {
            panic!("failing parsing id '{}'", id);
        };

        let colors = calc_min_colors(line.to_string());
        let (red, green, blue) = colors;
        let power = red * green * blue;

        total += power;

        println!(
            "game {} -> colors: {:?} t: {} p: {}",
            id, colors, total, power
        );
    }

    println!("total: {}", total);
}

fn calc_min_colors(line: String) -> (u32, u32, u32) {
    let (mut red, mut green, mut blue) = (0, 0, 0);

    for round in line.split("; ") {
        for hand in round.split(", ") {
            (red, green, blue) = get_min_required_colors(hand.to_string(), (red, green, blue));
        }
    }

    return (red, green, blue);
}

fn get_min_required_colors(hand: String, (red, green, blue): (u32, u32, u32)) -> (u32, u32, u32) {
    let Some((num, color)) = hand.split_once(" ") else {
        panic!("error on split hand");
    };
    let Ok(num)  = num.parse::<u32>() else {
        panic!("failing parsing hand count {}:{}", num, color);
    };

    match color {
        "red" => (max(red, num), green, blue),
        "green" => (red, max(green, num), blue),
        "blue" => (red, green, max(blue, num)),
        _ => {
            panic!("wrong color {}", color);
        }
    }
}

fn max(n1: u32, n2: u32) -> u32 {
    match n1 > n2 {
        true => n1,
        false => n2,
    }
}
