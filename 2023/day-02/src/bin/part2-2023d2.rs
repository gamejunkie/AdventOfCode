#[derive(strum_macros::Display)]
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
    println!("Day 2 - Part 2 Answer: {}", output);
    println!("============================");
}

fn calculate_product_from_min_cubes(row: &str) -> usize {
    let game: Vec<&str> = row.split(':').collect();
    let rounds: Vec<&str> = game[1].split(';').collect();
    println!("Game -------------------");
    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;
    for round in rounds {
        let cube_colors: Vec<&str> = round.split(',').collect();

        for cube_color_count in cube_colors {
            let each_color: Vec<&str> = cube_color_count.split(' ').collect();

            let mut color_count = 0;
            let mut cube_color = CubeColors::None;

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
                        cube_color = match item {
                            "red" => CubeColors::Red,
                            "green" => CubeColors::Green,
                            "blue" => CubeColors::Blue,
                            _ => panic!("Did not recognize the color"),
                        };
                    }
                }
            }

            println!("Color: {} Count {}", cube_color, color_count);
            match cube_color {
                CubeColors::Red => {
                    max_red = max_red.max(color_count);
                }
                CubeColors::Blue => {
                    max_blue = max_blue.max(color_count);
                }
                CubeColors::Green => {
                    max_green = max_green.max(color_count);
                }
                CubeColors::None => {
                    panic!("Did not recognize the color");
                }
            }
        }
    }
    (max_red.max(1) * max_blue.max(1) * max_green.max(1)) as usize
}

fn process(input: &str) -> String {
    let mut answer: usize = 0;

    for row in input.lines() {
        answer += calculate_product_from_min_cubes(row);
    }

    answer.to_string()
}
