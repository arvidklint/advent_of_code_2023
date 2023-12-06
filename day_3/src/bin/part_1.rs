fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Position {
    row: isize,
    col: isize,
}

#[derive(Debug)]
struct PartNumber {
    number: isize,
    positions: Vec<Position>,
}

fn is_symbol(maybe_symbol: Option<char>) -> bool {
    match maybe_symbol {
        None => false,
        Some('.') => false,
        // Some(c) if c.is_digit(10) => false,
        _ => true,
    }
}

fn get_position(input: &str, position: &Position) -> Option<char> {
    let lines = input.lines().collect::<Vec<&str>>();

    if position.row >= lines.len() as isize {
        return None;
    }
    if position.row < 0 {
        return None;
    }
    if position.col >= lines[0].len() as isize {
        return None;
    }
    if position.col < 0 {
        return None;
    }

    Some(lines[position.row as usize].chars().collect::<Vec<char>>()[position.col as usize])
}

fn check_for_symbol(input: &str, positions: &Vec<Position>) -> Option<char> {
    // let deltas: Vec<Position> = vec![
    //     Position { row: -1, col: -1 },
    //     Position { row: -1, col: 0 },
    //     Position { row: -1, col: 1 },
    //     Position { row: 0, col: -1 },
    //     Position { row: 0, col: 1 },
    //     Position { row: 1, col: -1 },
    //     Position { row: 1, col: 0 },
    //     Position { row: 1, col: 1 },
    // ];
    for position in positions {
        // for d in deltas.iter() {
        let maybe_symbol = get_position(input, position);

        if is_symbol(maybe_symbol) {
            return maybe_symbol;
        }
        // }
    }

    None
}

fn get_positions_around_number(start_position: Position, number: isize) -> Vec<Position> {
    let mut positions: Vec<Position> = vec![];

    for col in
        (start_position.col - 1)..(start_position.col + (number.to_string().len() as isize) + 1)
    {
        // add position over number
        positions.push(Position {
            row: start_position.row - 1,
            col,
        });

        // add position under number
        positions.push(Position {
            row: start_position.row + 1,
            col,
        });
    }

    // add positions to the sides
    positions.push(Position {
        row: start_position.row,
        col: start_position.col - 1,
    });
    positions.push(Position {
        row: start_position.row,
        col: start_position.col + (number.to_string().len() as isize),
    });

    positions
}

fn get_part_numbers(input: &str) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = vec![];
    for (row, line) in input.lines().enumerate() {
        let mut current_number_str = "".to_string();
        let mut current_start_position: Position = Position { row: 0, col: 0 };

        for (col, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                if current_number_str == "" {
                    current_start_position = Position {
                        row: row as isize,
                        col: col as isize,
                    };
                }
                current_number_str.push(char);
            }
            if col == line.len() - 1 {
                if current_number_str != "" {
                    let number = current_number_str.parse::<isize>().unwrap();
                    let part_number = PartNumber {
                        number,
                        positions: get_positions_around_number(current_start_position, number),
                    };
                    part_numbers.push(part_number);
                    current_number_str = "".to_string();
                    current_start_position = Position { row: 0, col: 0 };
                }
            } else if let Some(next_char) = line.chars().collect::<Vec<char>>().get(col + 1) {
                if !next_char.is_digit(10) {
                    if current_number_str != "" {
                        let number = current_number_str.parse::<isize>().unwrap();
                        let part_number = PartNumber {
                            number,
                            positions: get_positions_around_number(current_start_position, number),
                        };
                        part_numbers.push(part_number);
                        current_number_str = "".to_string();
                        current_start_position = Position { row: 0, col: 0 };
                    }
                }
            }
            // dbg!(&current_number_str);
        }
    }
    dbg!(&part_numbers);

    // remove all part numbers that don't have a symbol next to them
    part_numbers.retain(
        |part_number| match check_for_symbol(input, &part_number.positions) {
            Some(_) => true,
            None => false,
        },
    );

    part_numbers
}

fn part_1(input: &str) -> isize {
    let part_numbers = get_part_numbers(input);

    let mut sum = 0;
    for part_number in part_numbers {
        sum += part_number.number;
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = super::part_1(input);
        assert_eq!(result, 4361);
    }
}
