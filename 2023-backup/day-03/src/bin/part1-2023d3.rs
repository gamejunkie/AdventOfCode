fn main() {
    let input = include_str!("day3-input.txt");

    let output = process(input);

    println!("============================");
    println!("Day 3 - Part 1 Answer: {}", output);
    println!("============================");
}
#[derive(Debug)]
struct PartNumber {
    part_number: String,
    part_number_numeric: i32,
    start_position: usize,
    length: usize,
}

impl PartNumber {
    fn new(part_number: String) -> Self {
        PartNumber {
            part_number,
            part_number_numeric: 0,
            start_position: 0,
            length: 0,
        }
    }

    fn has_adjacent_symbol(&self, info_grid: &Vec<&str>, current_row_num: usize) -> bool {
        let start_pos = if self.start_position == 0 {
            0
        } else {
            (self.start_position - 1).max(0)
        };

        let end_pos = if self.start_position + self.length + 1 > info_grid[0].len() {
            info_grid[0].len()
        } else {
            self.start_position + self.length + 1
        };

        let top_line = if current_row_num == 0 {
            ""
        } else {
            &info_grid[current_row_num - 1][start_pos..end_pos]
        };

        let current_line = &info_grid[current_row_num][start_pos..end_pos];

        let bottom_line = if current_row_num == info_grid.len() - 1 {
            ""
        } else {
            &info_grid[current_row_num + 1][start_pos..end_pos]
        };

        for char in top_line.chars() {
            if is_symbol(char) {
                return true;
            }
        }

        for char in current_line.chars() {
            if is_symbol(char) {
                return true;
            }
        }

        for char in bottom_line.chars() {
            if is_symbol(char) {
                return true;
            }
        }
        false
    }
}

fn is_symbol(symbol: char) -> bool {
    if symbol.is_numeric() || symbol == '.' {
        return false;
    }
    true
}

fn get_part_numbers(row: &str) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut scanning_number = false;
    let mut part_number = PartNumber::new(String::new());
    for (position, character) in row.chars().enumerate() {
        if character.is_numeric() {
            scanning_number = true;
            part_number.part_number += &character.to_string();

            if position == row.len() - 1 {
                part_number.part_number_numeric = part_number.part_number.parse::<i32>().unwrap();
                part_number.start_position = position - part_number.part_number.len();
                part_number.length = part_number.part_number.len();
                part_numbers.push(part_number);
                part_number = PartNumber::new("".to_string());
            }
        } else if scanning_number {
            part_number.part_number_numeric = part_number.part_number.parse::<i32>().unwrap();
            part_number.start_position = position - part_number.part_number.len();
            part_number.length = part_number.part_number.len();
            part_numbers.push(part_number);
            part_number = PartNumber::new("".to_string());
            scanning_number = false;
        }
    }

    part_numbers
}

fn process(input: &str) -> String {
    let mut answer: i32 = 0;

    let mut info_grid = Vec::new();
    let mut part_numbers = Vec::new();

    for row in input.lines() {
        info_grid.push(row); // Record Grid
        part_numbers.push(get_part_numbers(row)); // Record part numbers for each row
    }

    for (row_num, part_numbers_for_row) in part_numbers.iter().enumerate() {
        for part_number in part_numbers_for_row {
            if part_number.has_adjacent_symbol(&info_grid, row_num) {
                answer += part_number.part_number_numeric;
            }
        }
    }

    answer.to_string()
}
