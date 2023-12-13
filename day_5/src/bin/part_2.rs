fn main() {
    let input = include_str!("../input.txt");
    let output = process(input);
    println!("output: {}", output);
}

#[derive(Debug)]
struct Range {
    source_start: isize,
    destination_start: isize,
    range: isize,
}

struct SeedRange {
    start: isize,
    range: isize,
}

fn extract_seeds(lines: &mut Vec<&str>) -> Vec<SeedRange> {
    let line = lines[0];
    let start = line.find(":").unwrap();
    let seeds = line[start + 1..].trim().to_string();
    let seeds = seeds.split(" ").collect::<Vec<&str>>();
    let seeds = seeds.chunks(2);
    let mut seed_ranges: Vec<SeedRange> = vec![];
    for seed in seeds {
        let start = seed[0].parse::<isize>().unwrap();
        let range = seed[1].parse::<isize>().unwrap();

        seed_ranges.push(SeedRange { start, range });
    }
    // .map(|s| s.parse::<isize>().unwrap())
    // .collect::<Vec<isize>>();
    lines.remove(0);
    lines.remove(0);

    seed_ranges
}

fn extract_ranges(lines: &mut Vec<&str>) -> Vec<Range> {
    let mut ranges: Vec<Range> = vec![];

    let end_index = lines.iter().position(|line| *line == "");

    lines.remove(0);

    let lines_slice = match end_index {
        Some(end_index) => lines.drain(0..end_index - 1),
        None => lines.drain(..),
    };

    for line in lines_slice {
        let map = line
            .split(" ")
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        ranges.push(Range {
            destination_start: map[0],
            source_start: map[1],
            range: map[2],
        });
    }

    if lines.len() > 0 {
        lines.remove(0);
    }

    ranges
}

fn map_range(number: isize, ranges: &Vec<Range>) -> isize {
    for range in ranges.iter() {
        if number >= range.source_start && number <= range.source_start + range.range {
            return number + range.destination_start - range.source_start;
        }
    }

    number
}

fn process(input: &str) -> isize {
    let mut lines = input.lines().collect::<Vec<&str>>();
    let seeds = extract_seeds(&mut lines);
    let seed_to_soil = extract_ranges(&mut lines);
    let soil_to_fertilizer = extract_ranges(&mut lines);
    let fertilizer_to_water = extract_ranges(&mut lines);
    let water_to_light = extract_ranges(&mut lines);
    let light_to_temperature = extract_ranges(&mut lines);
    let temperature_to_humidity = extract_ranges(&mut lines);
    let humidity_to_location = extract_ranges(&mut lines);

    let mut min = isize::MAX;
    for seed in seeds.iter() {
        let start = seed.start as usize;
        let end = start + seed.range as usize;
        for s in start..end {
            let soil = map_range(s as isize, &seed_to_soil);
            let fertilizer = map_range(soil, &soil_to_fertilizer);
            let water = map_range(fertilizer, &fertilizer_to_water);
            let light = map_range(water, &water_to_light);
            let temperature = map_range(light, &light_to_temperature);
            let humidity = map_range(temperature, &temperature_to_humidity);
            let location = map_range(humidity, &humidity_to_location);
            if min > location {
                min = location;
            }
        }
        println!("seed");
    }

    min
}

mod tests {
    #[test]
    fn part_2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let result = super::process(input);
        assert_eq!(result, 46);
    }
}
