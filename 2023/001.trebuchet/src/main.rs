use std::fs::read_to_string;

const NUMBER_LIST: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    // let mut total = 0;
    match read_to_string("./input") {
        Ok(file) => {
            let mut total: u32 = 0;

            for line in file.lines() {
                if !line.is_empty() {
                    let decimals = parse_line(line.to_string());

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

fn parse_line(line: String) -> Vec<u32> {
    replace_text_numbers(line)
        .chars()
        .filter_map(|d| d.to_digit(10))
        .collect()
}

fn replace_text_numbers(line: String) -> String {
    let mut new_line = "".to_string();
    let char_arr: Vec<char> = line.chars().into_iter().collect();

    for idx in 0..line.len() {
        let item = char_arr[idx];
        if item.is_ascii_digit() {
            new_line = format!("{}{}", new_line, item);
            continue;
        }

        let value = translate_textual_value(idx, &line);
        if value.len() > 0 {
            new_line = format!("{}{}", new_line, value);
        }
    }

    new_line
}

fn translate_textual_value(idx: usize, line: &str) -> String {
    for (num, value) in NUMBER_LIST.iter().enumerate() {
        if line[idx..].starts_with(value) {
            return num.to_string();
        }
    }

    "".to_string()
}
