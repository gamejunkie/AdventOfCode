static MAX_RED: i32 = 12;
static MAX_GREEN: i32 = 13;
static MAX_BLUE: i32 = 14;

enum CubeColors {
    Red,
    Blue,
    Green,
    None,
}

fn main() {
    let input = include_str!("day2-input.txt");

    let output = process(input);

    println!("============================");
    println!("Day 2 - Part 1 Answer: {}", output);
    println!("============================");
}

fn is_valid_game(row: &str) -> bool {
    let game: Vec<&str> = row.split(':').collect();
    let rounds: Vec<&str> = game[1].split(';').collect();
    for round in rounds {
        let cube_colors: Vec<&str> = round.split(',').collect();

        for cube_color_count in cube_colors {
            let each_color: Vec<&str> = cube_color_count.split(' ').collect();

            let mut color_count = 0;
            let mut dice_color = CubeColors::None;

            for item in each_color {
                if item.is_empty() {
                    continue;
                };

                match item.parse::<i32>() {
                    Ok(x) => {
                        color_count = x;
                        continue;
                    }
                    Err(_) => {
                        dice_color = match item {
                            "red" => CubeColors::Red,
                            "green" => CubeColors::Green,
                            "blue" => CubeColors::Blue,
                            _ => panic!("Did not recognize the color"),
                        };
                    }
                }

                match dice_color {
                    CubeColors::Red => {
                        if color_count > MAX_RED {
                            return false;
                        }
                    }
                    CubeColors::Blue => {
                        if color_count > MAX_BLUE {
                            return false;
                        }
                    }
                    CubeColors::Green => {
                        if color_count > MAX_GREEN {
                            return false;
                        }
                    }
                    CubeColors::None => {
                        panic!("Could not find a color");
                    }
                }
            }
        }
    }

    true
}

fn process(input: &str) -> String {
    let mut answer: usize = 0;

    for (row_num, row) in input.lines().enumerate() {
        if is_valid_game(row) {
            answer += row_num + 1;
        }
    }

    answer.to_string()
}
