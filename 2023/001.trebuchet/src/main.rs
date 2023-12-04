use std::fs::read_to_string;

fn main() {
    // let mut total = 0;
    match read_to_string("./input") {
        Ok(file) => {
            let mut total: u32 = 0;

            for line in file.lines() {
                if !line.is_empty() {
                    let decimals: Vec<u32> = line.chars().filter_map(|d| d.to_digit(10)).collect();

                    if decimals.len() < 1 {
                        continue;
                    }

                    let line_num: u32 = format!("{}{}", decimals[0], decimals[decimals.len() - 1])
                        .parse()
                        .unwrap();

                    total += line_num;
                }
            }

            println!("total: {}", total);
        }
        Err(e) => panic!("{}", e),
    }
}
