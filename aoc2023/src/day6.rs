pub mod part1 {
    #[derive(Debug)]
    struct Race {
        max_time: i32,
        max_distance: i32,
    }

    pub fn process() {
        let mut answer: i32 = 1;

        let input = include_str!("./testdata/day6-input.txt");

        let mut races = Vec::new();

        let mut times: Vec<i32> = Vec::new();
        let mut distances: Vec<i32> = Vec::new();

        for (row_num, row) in input.lines().enumerate() {
            if row_num == 0 {
                times = row
                    .split(' ')
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect();
            }

            if row_num == 1 {
                distances = row
                    .split(' ')
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect();
            }
        }

        let total_races = times.len();
        for x in 0..total_races {
            races.push(Race {
                max_time: times[x],
                max_distance: distances[x],
            });
        }

        for race in races {
            let mut wins = 0;
            for x in 1..race.max_time {
                let distance = x * (race.max_time - x);
                if distance > race.max_distance {
                    wins += 1;
                }
            }
            answer *= wins;
        }

        println!("============================");
        println!("Day 6 - Part 1 Answer: {}", answer);
        println!("============================");
    }
}

pub mod part2 {

    #[derive(Debug)]
    struct Race {
        max_time: u64,
        max_distance: u64,
    }

    pub fn process() {
        let mut answer: i32 = 1;

        let input = include_str!("./testdata/day6-input.txt");

        let mut races = Vec::new();

        let mut times: Vec<&str> = Vec::new();
        let mut distances: Vec<&str> = Vec::new();

        for (row_num, row) in input.lines().enumerate() {
            if row_num == 0 {
                times = row.split(' ').collect();
            }

            if row_num == 1 {
                distances = row.split(' ').collect();
            }
        }

        let max_time_str: String = times.iter().flat_map(|s| s.chars()).collect();
        let max_distance_str: String = distances.iter().flat_map(|s| s.chars()).collect();

        let max_time: Vec<&str> = max_time_str.split(':').collect();
        let max_distance: Vec<&str> = max_distance_str.split(':').collect();

        races.push(Race {
            max_time: max_time[1].parse().unwrap(),
            max_distance: max_distance[1].parse().unwrap(),
        });

        for race in races {
            let mut wins = 0;
            for x in 1..race.max_time {
                let distance = x * (race.max_time - x);
                if distance > race.max_distance {
                    wins += 1;
                }
            }
            answer *= wins;
        }

        println!("============================");
        println!("Day 6 - Part 2 Answer: {}", answer);
        println!("============================");
    }
}
