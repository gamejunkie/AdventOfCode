fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut answer: i32 = 0;
    for row in input.lines() {
        let mut found_first = false;

        let mut first_digit = '0';
        let mut second_digit = '0';
        for checkchar in row.chars() {
            if checkchar.is_numeric() {
                if !found_first {
                    first_digit = checkchar;
                    second_digit = first_digit; // Handle the case where there is only ever one digit in row.
                    found_first = true;
                } else {
                    second_digit = checkchar; // Track each digit as we progress through the row.
                }
            }
        }

        let temp_answer = (first_digit.to_string() + &second_digit.to_string())
            .parse::<i32>()
            .unwrap();

        answer += temp_answer;
    }

    answer.to_string()
}
