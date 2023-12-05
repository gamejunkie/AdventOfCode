fn main() {
    let input = include_str!("day4-input.txt");

    let output = process(input);

    println!("============================");
    println!("Day 4 - Part 2 Answer: {}", output);
    println!("============================");
}
#[derive(Debug, Clone)]
struct PartNumber {
    part_number: String,
    part_number_numeric: i32,
    start_position: (usize, usize),
    length: usize,
}

impl PartNumber {
    fn new(part_number: String) -> Self {
        PartNumber {
            part_number,
            part_number_numeric: 0,
            start_position: (0, 0),
            length: 0,
        }
    }

    fn is_adjacent_to_gear(&self, gear_position: (usize, usize)) -> bool {
        // Check verticality
        let mut vertical_range = (0, gear_position.0 + 1);
        if gear_position.0 != 0 {
            vertical_range.0 = gear_position.0 - 1;
        }

        if self.start_position.0 < vertical_range.0 {
            return false;
        }

        if self.start_position.0 > vertical_range.1 {
            return false;
        }

        // Check horizontal
        let mut horiz_range = (0, self.start_position.1 + self.length);
        if self.start_position.1 != 0 {
            horiz_range.0 = self.start_position.1 - 1;
        }

        if gear_position.1 < horiz_range.0 {
            return false;
        }

        if gear_position.1 > horiz_range.1 {
            return false;
        }

        true
    }
}

fn get_part_numbers(row: &str, row_num: usize) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut scanning_number = false;
    let mut part_number = PartNumber::new(String::new());
    for (position, character) in row.chars().enumerate() {
        if character.is_numeric() {
            scanning_number = true;
            part_number.part_number += &character.to_string();

            if position == row.len() - 1 {
                part_number.part_number_numeric = part_number.part_number.parse::<i32>().unwrap();
                part_number.start_position = (row_num, position - part_number.part_number.len());
                part_number.length = part_number.part_number.len();
                part_numbers.push(part_number);
                part_number = PartNumber::new("".to_string());
            }
        } else if scanning_number {
            part_number.part_number_numeric = part_number.part_number.parse::<i32>().unwrap();
            part_number.start_position = (row_num, position - part_number.part_number.len());
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

    for (row_number, row) in input.lines().enumerate() {
        info_grid.push(row); // Record Grid
        part_numbers.push(get_part_numbers(row, row_number)); // Record part numbers for each row
    }

    // Locate the position of each gear (*)
    let mut gears = Vec::new();
    for (row_num, row) in info_grid.iter().enumerate() {
        for (position, char) in row.chars().enumerate() {
            if char == '*' {
                gears.push((row_num, position));
            }
        }
    }

    for gear in gears {
        let mut adjacent_parts = Vec::new();
        for part_number_list in part_numbers.clone() {
            for part_number in part_number_list.clone() {
                if part_number.is_adjacent_to_gear(gear) {
                    adjacent_parts.push(part_number.part_number_numeric);
                }
            }
        }
        if adjacent_parts.len() == 2 {
            answer += adjacent_parts[0] * adjacent_parts[1];
        }
    }

    answer.to_string()
}
