pub mod part1 {

    fn main_logic(row: &str) -> bool {
        true
    }

    pub fn process() {
        let mut answer: usize = 0;

        let input = include_str!("./testdata/day?-input.txt");

        for (row_num, row) in input.lines().enumerate() {
            if main_logic(row) {
                answer += row_num + 1;
            }
        }

        println!("============================");
        println!("Day ? - Part 1 Answer: {}", answer);
        println!("============================");
    }
}

pub mod part2 {

    fn main_logic(row: &str) -> bool {
        true
    }

    pub fn process() {
        let mut answer: usize = 0;

        let input = include_str!("./testdata/day?-input.txt");

        for (row_num, row) in input.lines().enumerate() {
            if main_logic(row) {
                answer += row_num + 1;
            }
        }

        println!("============================");
        println!("Day ? - Part 2 Answer: {}", answer);
        println!("============================");
    }
}
