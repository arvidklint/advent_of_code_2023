fn main() {
    let input = include_str!("../input.txt");
    let output = part_2(input);
    dbg!(output);
}

#[derive(Debug)]
struct GameSet {
    red: isize,
    green: isize,
    blue: isize,
}

fn parse_set(set: &str) -> GameSet {
    let cube_amounts: Vec<&str> = set.split(", ").collect();

    let mut game_set = GameSet {
        red: 0,
        green: 0,
        blue: 0
    };

    for cube_amount in cube_amounts {
        let amount = cube_amount.split(" ").collect::<Vec<&str>>()[0];
        if cube_amount.ends_with("red") {
            game_set.red = amount.parse::<isize>().unwrap();
        }
        if cube_amount.ends_with("green") {
            game_set.green = amount.parse::<isize>().unwrap();
        }
        if cube_amount.ends_with("blue") {
            game_set.blue = amount.parse::<isize>().unwrap();
        }
    }
    
    game_set
}

fn get_sets(line: &str) -> Vec<GameSet> {
    // remove game label
    let game_label_end = line.find(": ").unwrap() + 2;
    let line = line[game_label_end..].to_string();

    let sets: Vec<&str> = line.split("; ").collect();

    sets.iter().map(|set| parse_set(set)).collect::<Vec<GameSet>>()
}

fn part_2(input: &str) -> isize {
    let mut sum = 0;
    for line in input.lines() {
        let sets = get_sets(line);

        let mut min_values = GameSet {
            red: 0,
            green: 0,
            blue: 0
        };

        for set in sets {
            if set.red > min_values.red {
                min_values.red = set.red
            }
            if set.green > min_values.green {
                min_values.green = set.green
            }
            if set.blue > min_values.blue {
                min_values.blue = set.blue
            }
        }

        let power = min_values.red * min_values.green * min_values.blue;

        sum += power;
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = super::part_2(input);
        assert_eq!(result, 2286);
    }
}