// Assumptions:
// - The order of maps is consistent from seed-to-soil to humidity-to-location
// -
pub mod part1 {

    use std::ops::RangeInclusive;

    #[derive(Debug)]
    struct RangeAndOffset {
        range: RangeInclusive<usize>,
        dest_start: usize,
    }

    fn get_mapping(val: usize, data_map: &Vec<RangeAndOffset>) -> usize {
        for dm in data_map {
            if dm.range.contains(&val) {
                return (val - dm.range.start()) + dm.dest_start;
            }
        }

        val
    }

    pub fn process() {
        let input = include_str!("./testdata/day5-input.txt");
        let mut seeds: Vec<usize> = Vec::new();
        let mut seed_to_soil: Vec<RangeAndOffset> = Vec::new();
        let mut soil_to_fertilizer: Vec<RangeAndOffset> = Vec::new();
        let mut fertilizer_to_water: Vec<RangeAndOffset> = Vec::new();
        let mut water_to_light: Vec<RangeAndOffset> = Vec::new();
        let mut light_to_temperature: Vec<RangeAndOffset> = Vec::new();
        let mut temperature_to_humidity: Vec<RangeAndOffset> = Vec::new();
        let mut humidity_to_location: Vec<RangeAndOffset> = Vec::new();

        let mut current_map: &mut Vec<RangeAndOffset> = &mut seed_to_soil;

        for (row_num, row) in input.lines().enumerate() {
            if row_num == 0 {
                seeds = row
                    .split(' ')
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();

                continue;
            }

            if row.is_empty() {
                continue;
            }

            if row.contains("map:") {
                let temp: Vec<&str> = row.split(' ').collect();

                current_map = match temp[0] {
                    "seed-to-soil" => &mut seed_to_soil,
                    "soil-to-fertilizer" => &mut soil_to_fertilizer,
                    "fertilizer-to-water" => &mut fertilizer_to_water,
                    "water-to-light" => &mut water_to_light,
                    "light-to-temperature" => &mut light_to_temperature,
                    "temperature-to-humidity" => &mut temperature_to_humidity,
                    "humidity-to-location" => &mut humidity_to_location,
                    _ => panic!("Bad Section"),
                };

                continue;
            }

            let temp: Vec<usize> = row
                .split(' ')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();

            assert!(temp.len() == 3);

            current_map.push(RangeAndOffset {
                range: temp[1]..=(temp[1] + temp[2] - 1),
                dest_start: temp[0],
            });
        }

        let mut locations = Vec::new();
        for seed in seeds {
            let soil_mapping = get_mapping(seed, &seed_to_soil);
            let fertilizer_mapping = get_mapping(soil_mapping, &soil_to_fertilizer);
            let water_mapping = get_mapping(fertilizer_mapping, &fertilizer_to_water);
            let light_mapping = get_mapping(water_mapping, &water_to_light);
            let temperature_mapping = get_mapping(light_mapping, &light_to_temperature);
            let humidity_mapping = get_mapping(temperature_mapping, &temperature_to_humidity);
            let location_mapping = get_mapping(humidity_mapping, &humidity_to_location);
            locations.push(location_mapping);
        }

        println!("============================");
        println!("Day 5 - Part 1 Answer: {}", locations.iter().min().unwrap());
        println!("============================");
    }
}

pub mod part2 {

    use std::ops::RangeInclusive;

    #[derive(Debug)]
    struct RangeAndOffset {
        range: RangeInclusive<usize>,
        dest_start: usize,
    }

    fn get_mapping(val: usize, data_map: &Vec<RangeAndOffset>) -> usize {
        for dm in data_map {
            if dm.range.contains(&val) {
                return (val - dm.range.start()) + dm.dest_start;
            }
        }

        val
    }

    pub fn process() {
        let input = include_str!("./testdata/day5-input.txt");
        let mut seeds: Vec<usize> = Vec::new();
        let mut seed_to_soil: Vec<RangeAndOffset> = Vec::new();
        let mut soil_to_fertilizer: Vec<RangeAndOffset> = Vec::new();
        let mut fertilizer_to_water: Vec<RangeAndOffset> = Vec::new();
        let mut water_to_light: Vec<RangeAndOffset> = Vec::new();
        let mut light_to_temperature: Vec<RangeAndOffset> = Vec::new();
        let mut temperature_to_humidity: Vec<RangeAndOffset> = Vec::new();
        let mut humidity_to_location: Vec<RangeAndOffset> = Vec::new();

        let mut current_map: &mut Vec<RangeAndOffset> = &mut seed_to_soil;

        for (row_num, row) in input.lines().enumerate() {
            if row_num == 0 {
                let seed_ranges: Vec<usize> = row
                    .split(' ')
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();
                let mut seed_range_iter = seed_ranges.iter().peekable();
                loop {
                    let seed = *seed_range_iter.next().unwrap();
                    seeds.push(seed);
                    let num = &seed_range_iter.next().unwrap();
                    for x in 1..**num {
                        seeds.push(seed + x);
                    }

                    if seed_range_iter.peek().is_none() {
                        break;
                    }
                }

                continue;
            }

            if row.is_empty() {
                continue;
            }

            if row.contains("map:") {
                let temp: Vec<&str> = row.split(' ').collect();

                current_map = match temp[0] {
                    "seed-to-soil" => &mut seed_to_soil,
                    "soil-to-fertilizer" => &mut soil_to_fertilizer,
                    "fertilizer-to-water" => &mut fertilizer_to_water,
                    "water-to-light" => &mut water_to_light,
                    "light-to-temperature" => &mut light_to_temperature,
                    "temperature-to-humidity" => &mut temperature_to_humidity,
                    "humidity-to-location" => &mut humidity_to_location,
                    _ => panic!("Bad Section"),
                };

                continue;
            }

            let temp: Vec<usize> = row
                .split(' ')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();

            assert!(temp.len() == 3);

            current_map.push(RangeAndOffset {
                range: temp[1]..=(temp[1] + temp[2] - 1),
                dest_start: temp[0],
            });
        }

        let mut locations = Vec::new();
        let mut total_to_process = seeds.len();
        for seed in seeds {
            let soil_mapping = get_mapping(seed, &seed_to_soil);
            let fertilizer_mapping = get_mapping(soil_mapping, &soil_to_fertilizer);
            let water_mapping = get_mapping(fertilizer_mapping, &fertilizer_to_water);
            let light_mapping = get_mapping(water_mapping, &water_to_light);
            let temperature_mapping = get_mapping(light_mapping, &light_to_temperature);
            let humidity_mapping = get_mapping(temperature_mapping, &temperature_to_humidity);
            let location_mapping = get_mapping(humidity_mapping, &humidity_to_location);
            locations.push(location_mapping);
            total_to_process -= 1;
            if total_to_process % 50000000 == 0 {
                println!("To Process: {}", total_to_process);
            }
        }

        println!("============================");
        println!("Day 5 - Part 2 Answer: {}", locations.iter().min().unwrap());
        println!("============================");
    }
}
