use regex::Regex;

use crate::file_reader::read_file_to_string;

fn parse_corrupted_data(text: &str, handle_does: bool) -> Vec<String> {
    let text = {
        if handle_does {
            let re =
                Regex::new(r"(^(([\s\S])*?)don't\(\)|do\(\)([\s\S]*?)don't\(\)|do\(\)[\s\S]*)")
                    .expect("Failed to create Does regex");
            let a: String = re.find_iter(text).map(|r| r.as_str()).collect();
            a
        } else {
            text.to_string()
        }
    };

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").expect("Failed to create the regex");

    let result: Vec<String> = re
        .find_iter(&text)
        .map(|r| r.as_str().to_string())
        .collect();

    result
}

fn process_instructions(instructions: Vec<String>) -> i32 {
    let re = Regex::new(r"[0-9]{1,3}").expect("Failed to create process Regex");

    let mut sum = 0;

    for instruction in instructions {
        let values: Vec<i32> = re
            .find_iter(&instruction)
            .map(|i| {
                let number = i.as_str().to_string().parse::<i32>();
                number.unwrap()
            })
            .collect();

        sum += values[0] * values[1];
    }

    sum
}

/// Checks the distance between two list of unique ids
fn part_1(text: &str) -> i32 {
    let parsed_instructions = parse_corrupted_data(text, false);
    let result = process_instructions(parsed_instructions);

    result
}

fn part_2(text: &str) -> i32 {
    let parsed_instructions = parse_corrupted_data(text, true);
    let result = process_instructions(parsed_instructions);

    result
}

pub fn third_day_challenge() {
    let input = read_file_to_string("day_3.txt");

    // println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_corrupted_data_works() {
        let expected = vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"];
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(expected, parse_corrupted_data(input, false));
    }

    #[test]
    fn parse_corrupted_data_with_does_and_donts() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = vec!["mul(2,4)", "mul(8,5)"];

        assert_eq!(expected, parse_corrupted_data(input, true));
    }

    #[test]
    fn process_instructions_works() {
        let expected = 161;
        let input = vec![
            "mul(2,4)".to_string(),
            "mul(5,5)".to_string(),
            "mul(11,8)".to_string(),
            "mul(8,5)".to_string(),
        ];

        assert_eq!(expected, process_instructions(input));
    }
}
