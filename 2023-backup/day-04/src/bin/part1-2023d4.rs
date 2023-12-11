fn main() {
    let input = include_str!("day4-input.txt");

    let output = process(input);

    println!("============================");
    println!("Day 4 - Part 1 Answer: {}", output);
    println!("============================");
}

fn process(input: &str) -> String {
    let mut answer: i32 = 0;

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

    answer.to_string()
}
