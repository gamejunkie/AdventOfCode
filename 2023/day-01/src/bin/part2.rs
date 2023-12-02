static POSSIBLE_NUMBER_SET: &[(char, &str)] = &[
    ('1', "one"),
    ('2', "two"),
    ('3', "three"),
    ('4', "four"),
    ('5', "five"),
    ('6', "six"),
    ('7', "seven"),
    ('8', "eight"),
    ('9', "nine"),
];
fn main() {
    let input = include_str!("day1-input.txt");
    let output = process(input);
    println!("============================");
    println!("Day 1 - Part 2 Answer: {}", output);
    println!("============================");
}

fn find_min_max_digits(slice: &str) -> i32 {
    let mut final_vec: Vec<(usize, &str)> = Vec::new();

    for (charnum, strnum) in POSSIBLE_NUMBER_SET {
        final_vec.append(&mut slice.match_indices(*charnum).collect());
        final_vec.append(&mut slice.match_indices(strnum).collect());
    }

    let mut max_num = (0, "");
    let mut min_num = (slice.len(), "");

    for (index, matched) in final_vec {
        if index >= max_num.0 {
            max_num = (index, matched);
        }
        if index <= min_num.0 {
            min_num = (index, matched);
        }
    }

    let leftnum = POSSIBLE_NUMBER_SET
        .iter()
        .find(|&&val| val.0.to_string() == min_num.1 || val.1 == min_num.1);

    let rightnum = POSSIBLE_NUMBER_SET
        .iter()
        .find(|&&val| val.0.to_string() == max_num.1 || val.1 == max_num.1);

    ((leftnum.unwrap().0).to_string() + &(rightnum.unwrap().0).to_string())
        .parse::<i32>()
        .unwrap()
}

fn process(input: &str) -> String {
    let mut answer: i32 = 0;

    for row in input.lines() {
        answer += find_min_max_digits(row);
    }

    answer.to_string()
}
