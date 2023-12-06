fn main() {
    let input = include_str!("../input.txt");
    let output = part_2(input);
    dbg!(output);
}

struct TextNumber<'a> {
    char: char,
    name: &'a str,
}

fn part_2(input: &str) -> isize {
    let mut sum = 0;

    let mapper = [
        TextNumber {
            name: "one",
            char: '1'
        },
        TextNumber {
            name: "two",
            char: '2'
        },
        TextNumber {
            name: "three",
            char: '3'
        },
        TextNumber {
            name: "four",
            char: '4'
        },
        TextNumber {
            name: "five",
            char: '5'
        },
        TextNumber {
            name: "six",
            char: '6'
        },
        TextNumber {
            name: "seven",
            char: '7'
        },
        TextNumber {
            name: "eight",
            char: '8'
        },
        TextNumber {
            name: "nine",
            char: '9'
        },
        TextNumber {
            name: "1",
            char: '1'
        },
        TextNumber {
            name: "2",
            char: '2',
        },
        TextNumber {
            name: "3",
            char: '3'
        },
        TextNumber {
            name: "4",
            char: '4'
        },
        TextNumber {
            name: "5",
            char: '5'
        },
        TextNumber {
            name: "6",
            char: '6'
        },
        TextNumber {
            name: "7",
            char: '7'
        },
        TextNumber {
            name: "8",
            char: '8'
        },
        TextNumber {
            name: "9",
            char: '9'
        },
    ];

    for line in input.lines() {
        let mut min_index = usize::MAX;
        let mut min = '0';
        let mut max_index = usize::MIN;
        let mut max = '0';

        for text_number in mapper.iter() {
            let indexes: Vec<usize> = line.match_indices(text_number.name).map(|(i, _)| i).collect();
            for index in indexes {
                if index <= min_index {
                    min_index = index;
                    min = text_number.char;
                }
                if index >= max_index {
                    max_index = index;
                    max = text_number.char;
                }
            }
        }

        let mut number = min.to_string();
        number.push(max);
        let digit = number.parse::<isize>().unwrap();

        sum += digit as isize;
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = super::part_2(input);
        assert_eq!(result, 281);
    }
}
