use crate::file_reader::read_file_to_string;

type Grid = Vec<Vec<String>>;

fn parse_string_into_grid(text: &str) -> Grid {
    let mut grid: Grid = vec![vec![]];

    let mut idx = 0;
    text.chars().for_each(|c| {
        if c == '\n' {
            idx += 1;
            grid.push(vec![]);
        } else {
            grid[idx].push(c.to_string());
        }
    });

    grid
}

struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

struct WordMatchValidator<'a> {
    grid: &'a Grid,
    start_point: Point,
    word: &'a str,
}

impl<'a> WordMatchValidator<'a> {
    fn validate(&self, direction: &Direction) -> bool {
        let mut chars_matched = 0;
        for (word_idx, char) in self.word.chars().enumerate() {
            let position_to_check = match direction {
                Direction::Right => Point {
                    x: self.start_point.x + (word_idx as i32),
                    y: self.start_point.y,
                },
                Direction::Left => Point {
                    x: self.start_point.x - (word_idx as i32),
                    y: self.start_point.y,
                },
                Direction::Up => Point {
                    x: self.start_point.x,
                    y: self.start_point.y + (word_idx as i32),
                },
                Direction::Down => Point {
                    x: self.start_point.x,
                    y: self.start_point.y - (word_idx as i32),
                },
                Direction::DownLeft => Point {
                    x: self.start_point.x - (word_idx as i32),
                    y: self.start_point.y - (word_idx as i32),
                },
                Direction::DownRight => Point {
                    x: self.start_point.x + (word_idx as i32),
                    y: self.start_point.y - (word_idx as i32),
                },
                Direction::UpLeft => Point {
                    x: self.start_point.x - (word_idx as i32),
                    y: self.start_point.y + (word_idx as i32),
                },
                Direction::UpRight => Point {
                    x: self.start_point.x + (word_idx as i32),
                    y: self.start_point.y + (word_idx as i32),
                },
            };

            if position_to_check.y < 0 || position_to_check.x < 0 {
                return false;
            }
            let line_at_position = self.grid.get(position_to_check.y as usize);

            if line_at_position.is_none() {
                return false;
            }

            let line_at_position = line_at_position.unwrap();
            let char_at_position = line_at_position.get(position_to_check.x as usize);

            if char_at_position.is_none() {
                return false;
            }
            let char_at_position = char_at_position.unwrap();

            if char_at_position == &char.to_string() {
                chars_matched += 1;
            }
        }

        chars_matched == self.word.len()
    }
}

fn word_counter(text: &str, word: &str) -> i32 {
    let grid: Grid = parse_string_into_grid(text);
    let mut count = 0;

    let word_string = word.to_string();

    let first_word_char = word_string.get(0..1).unwrap();
    let directions_to_match: Vec<Direction> = vec![
        Direction::Up,
        Direction::Down,
        Direction::Right,
        Direction::Left,
        Direction::UpLeft,
        Direction::UpRight,
        Direction::DownLeft,
        Direction::DownRight,
    ];

    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == first_word_char {
                let start_point = Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };
                let validator = WordMatchValidator {
                    grid: &grid,
                    start_point,
                    word,
                };

                for d in directions_to_match.iter() {
                    let word_found = validator.validate(d);
                    if word_found {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn x_mas_counter(text: &str) -> i32 {
    let grid: Grid = parse_string_into_grid(text);
    let mut count = 0;

    let word_string = "MAS".to_string();

    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == "A" {
                let x_parsed: i32 = x.try_into().unwrap();
                let y_parsed: i32 = y.try_into().unwrap();
                let top_left = Point {
                    x: x_parsed - 1,
                    y: y_parsed + 1,
                };
                let top_right = Point {
                    x: x_parsed + 1,
                    y: y_parsed + 1,
                };
                let bottom_left = Point {
                    x: x_parsed - 1,
                    y: y_parsed - 1,
                };
                let bottom_right = Point {
                    x: x_parsed + 1,
                    y: y_parsed - 1,
                };

                let places_to_validate = vec![
                    (top_left, Direction::DownRight),
                    (top_right, Direction::DownLeft),
                    (bottom_left, Direction::UpRight),
                    (bottom_right, Direction::UpLeft),
                ];

                let mut c = 0;
                for (point, direction) in places_to_validate {
                    let validator = WordMatchValidator {
                        grid: &grid,
                        start_point: point,
                        word: &word_string,
                    };
                    let word_found = validator.validate(&direction);
                    if word_found {
                        c += 1;
                    }
                }

                if c == 2 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part_1(text: &str) -> i32 {
    word_counter(text, "XMAS")
}

fn part_2(text: &str) -> i32 {
    x_mas_counter(text)
}

pub fn fourth_day_challenge() {
    let input = read_file_to_string("day_4.txt");

    // println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_counter_works() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        let amount_found = word_counter(input, "XMAS");
        assert_eq!(amount_found, 18);
    }

    #[test]
    fn word_x_mas_counter_works() {
        let input = r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#;

        let amount_found = x_mas_counter(input);
        assert_eq!(amount_found, 9);
    }
}
