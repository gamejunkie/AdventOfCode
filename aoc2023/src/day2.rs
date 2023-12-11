pub mod part1 {

    static MAX_RED: i32 = 12;
    static MAX_GREEN: i32 = 13;
    static MAX_BLUE: i32 = 14;

    enum CubeColors {
        Red,
        Blue,
        Green,
        None,
    }

    fn is_valid_game(row: &str) -> bool {
        let game: Vec<&str> = row.split(':').collect();
        let rounds: Vec<&str> = game[1].split(';').collect();
        for round in rounds {
            let cube_colors: Vec<&str> = round.split(',').collect();

            for cube_color_count in cube_colors {
                let each_color: Vec<&str> = cube_color_count.split(' ').collect();

                let mut color_count = 0;
                #[allow(unused_assignments)]
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

    pub fn process() {
        let mut answer: usize = 0;

        let input = include_str!("./testdata/day2-input.txt");

        for (row_num, row) in input.lines().enumerate() {
            if is_valid_game(row) {
                answer += row_num + 1;
            }
        }

        println!("============================");
        println!("Day 2 - Part 1 Answer: {}", answer);
        println!("============================");
    }
}

pub mod part2 {

    #[derive(strum_macros::Display)]
    enum CubeColors {
        Red,
        Blue,
        Green,
        None,
    }
    fn calculate_product_from_min_cubes(row: &str) -> usize {
        let game: Vec<&str> = row.split(':').collect();
        let rounds: Vec<&str> = game[1].split(';').collect();
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
    pub fn process() {
        let mut answer: usize = 0;

        let input = include_str!("./testdata/day2-input.txt");

        for row in input.lines() {
            answer += calculate_product_from_min_cubes(row);
        }

        println!("============================");
        println!("Day 2 - Part 2 Answer: {}", answer);
        println!("============================");
    }
}
