pub mod part1 {

    pub fn process() {
        let mut answer: i32 = 0;

        let input = include_str!("./testdata/day4-input.txt");

        let mut winning_numbers: Vec<Vec<&str>> = Vec::new();
        let mut my_numbers: Vec<Vec<&str>> = Vec::new();

        for row in input.lines() {
            let game: Vec<&str> = row.split(": ").collect();
            let card_data: Vec<&str> = game[1].split(" | ").collect();
            let win_list: Vec<&str> = card_data[0].split(' ').filter(|s| !s.is_empty()).collect();
            let my_list: Vec<&str> = card_data[1].split(' ').filter(|s| !s.is_empty()).collect();

            winning_numbers.push(win_list);
            my_numbers.push(my_list);
        }

        for (row_num, my_number_list) in my_numbers.iter().enumerate() {
            let winning_matches: Vec<&str> = my_number_list
                .iter()
                .filter(|s| winning_numbers[row_num].contains(s))
                .cloned()
                .collect();

            let base: f64 = 2.0;
            answer += base.powf(winning_matches.len() as f64 - 1.0) as i32;
        }

        println!("============================");
        println!("Day 4 - Part 1 Answer: {}", answer);
        println!("============================");
    }
}

pub mod part2 {

    pub fn process() {
        let input = include_str!("./testdata/day4-input.txt");

        let mut winning_numbers: Vec<Vec<&str>> = Vec::new();
        let mut my_numbers: Vec<Vec<&str>> = Vec::new();

        for row in input.lines() {
            let game: Vec<&str> = row.split(": ").collect();
            let card_data: Vec<&str> = game[1].split(" | ").collect();
            let win_list: Vec<&str> = card_data[0].split(' ').filter(|s| !s.is_empty()).collect();
            let my_list: Vec<&str> = card_data[1].split(' ').filter(|s| !s.is_empty()).collect();

            winning_numbers.push(win_list);
            my_numbers.push(my_list);
        }

        let mut card_copy_count = vec![1; my_numbers.len()];

        for (row_num, my_number_list) in my_numbers.iter().enumerate() {
            let winning_matches: Vec<&str> = my_number_list
                .iter()
                .filter(|s| winning_numbers[row_num].contains(s))
                .cloned()
                .collect();

            let total_matches = winning_matches.len();

            for number in 1..=total_matches {
                card_copy_count[row_num + number] += card_copy_count[row_num];
            }
        }
        let count: usize = card_copy_count.iter().sum();

        println!("============================");
        println!("Day 4 - Part 2 Answer: {}", count);
        println!("============================");
    }
}
