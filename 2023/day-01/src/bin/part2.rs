fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut answer: i32 = 0;

    for (row_num, row) in input.lines().enumerate() {
        let mut found_first = false;
        let mut first_digit = '.';
        let mut second_digit = '.';
        let mut pos_of_next_char_to_check = 0;
        for (position_in_row, checkchar) in row.chars().enumerate() {
            if position_in_row == pos_of_next_char_to_check {
                if checkchar.is_numeric() {
                    if !found_first {
                        first_digit = checkchar;
                        second_digit = first_digit; // Handle the case where there is only ever one digit in row.
                        found_first = true;
                    } else {
                        second_digit = checkchar; // Track each digit as we progress through the row.
                    }
                    pos_of_next_char_to_check += 1;
                } else {
                    // Check the character and see if it falls within the first char of the string-based digits.
                    let remaining_str_length = row.len() - position_in_row;
                    match checkchar {
                        'o' => {
                            let val = ('1', "one");
                            if remaining_str_length >= val.1.len()
                                && &row[position_in_row..position_in_row + val.1.len()] == val.1
                            {
                                if !found_first {
                                    first_digit = val.0;
                                    found_first = true;
                                    second_digit = first_digit;
                                } else {
                                    second_digit = val.0;
                                }
                                pos_of_next_char_to_check += val.1.len();
                            } else {
                                pos_of_next_char_to_check += 1;
                            }
                            continue;
                        }

                        't' => {
                            let val = ('2', "two");
                            if remaining_str_length >= val.1.len()
                                && &row[position_in_row..position_in_row + val.1.len()] == val.1
                            {
                                if !found_first {
                                    first_digit = val.0;
                                    found_first = true;
                                    second_digit = first_digit;
                                } else {
                                    second_digit = val.0;
                                }
                                pos_of_next_char_to_check += val.1.len();
                                continue;
                            } else {
                                let val = ('3', "three");
                                if remaining_str_length >= val.1.len()
                                    && &row[position_in_row..position_in_row + val.1.len()] == val.1
                                {
                                    if !found_first {
                                        first_digit = val.0;
                                        found_first = true;
                                        second_digit = first_digit;
                                    } else {
                                        second_digit = val.0;
                                    }
                                    pos_of_next_char_to_check += val.1.len();
                                    continue;
                                }
                            }

                            pos_of_next_char_to_check += 1;
                            continue;
                        }
                        'f' => {
                            let val = ('4', "four");
                            if remaining_str_length >= val.1.len()
                                && &row[position_in_row..position_in_row + val.1.len()] == val.1
                            {
                                if !found_first {
                                    first_digit = val.0;
                                    found_first = true;
                                    second_digit = first_digit;
                                } else {
                                    second_digit = val.0;
                                }
                                pos_of_next_char_to_check += val.1.len();
                                continue;
                            } else {
                                let val = ('5', "five");
                                if remaining_str_length >= val.1.len()
                                    && &row[position_in_row..position_in_row + val.1.len()] == val.1
                                {
                                    if !found_first {
                                        first_digit = val.0;
                                        found_first = true;
                                        second_digit = first_digit;
                                    } else {
                                        second_digit = val.0;
                                    }
                                    pos_of_next_char_to_check += val.1.len();
                                    continue;
                                }
                            }

                            pos_of_next_char_to_check += 1;
                            continue;
                        }
                        's' => {
                            let val = ('6', "six");
                            if remaining_str_length >= val.1.len()
                                && &row[position_in_row..position_in_row + val.1.len()] == val.1
                            {
                                if !found_first {
                                    first_digit = val.0;
                                    found_first = true;
                                    second_digit = first_digit;
                                } else {
                                    second_digit = val.0;
                                }
                                pos_of_next_char_to_check += val.1.len();
                                continue;
                            } else {
                                let val = ('7', "seven");
                                if remaining_str_length >= val.1.len()
                                    && &row[position_in_row..position_in_row + val.1.len()] == val.1
                                {
                                    if !found_first {
                                        first_digit = val.0;
                                        found_first = true;
                                        second_digit = first_digit;
                                    } else {
                                        second_digit = val.0;
                                    }
                                    pos_of_next_char_to_check += val.1.len();
                                    continue;
                                }
                            }

                            pos_of_next_char_to_check += 1;
                            continue;
                        }
                        'e' => {
                            let val = ('8', "eight");
                            if remaining_str_length >= val.1.len()
                                && &row[position_in_row..position_in_row + val.1.len()] == val.1
                            {
                                if !found_first {
                                    first_digit = val.0;
                                    found_first = true;
                                    second_digit = first_digit;
                                } else {
                                    second_digit = val.0;
                                }
                                pos_of_next_char_to_check += val.1.len();
                            } else {
                                pos_of_next_char_to_check += 1;
                            }
                            continue;
                        }
                        'n' => {
                            let val = ('9', "nine");
                            if remaining_str_length >= val.1.len()
                                && &row[position_in_row..position_in_row + val.1.len()] == val.1
                            {
                                if !found_first {
                                    first_digit = val.0;
                                    found_first = true;
                                    second_digit = first_digit;
                                } else {
                                    second_digit = val.0;
                                }
                                pos_of_next_char_to_check += val.1.len();
                            } else {
                                pos_of_next_char_to_check += 1;
                            }
                            continue;
                        }
                        _ => {
                            pos_of_next_char_to_check += 1;
                        }
                    };
                }
            }
        }

        let temp_answer = (first_digit.to_string() + &second_digit.to_string())
            .parse::<i32>()
            .unwrap();
        answer += temp_answer;
        println!("{}:{}:{}", row_num, temp_answer, answer);
    }

    answer.to_string()
}
