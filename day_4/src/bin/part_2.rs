fn main() {
    let input = include_str!("../input.txt");
    let output = process(input);
    dbg!(output);
}

fn get_winning_numbers(line: &str) -> Vec<isize> {
    let start = line.find(":").unwrap();
    let end = line.find("|").unwrap();
    let numbers = line[start+1..end].trim().to_string();

    numbers.split(" ").filter(|n| *n != "").map(|number| number.parse::<isize>().unwrap()).collect::<Vec<isize>>()
}

fn get_my_numbers(line: &str) -> Vec<isize> {
    let start = line.find("|").unwrap();
    let numbers = line[start+1..].trim().to_string();

    numbers.split(" ").filter(|n| *n != "").map(|number| number.parse::<isize>().unwrap()).collect::<Vec<isize>>()
}

fn process(input: &str) -> isize {
    let wins_per_card = input.lines().map(|line| {
        let winning_numbers = get_winning_numbers(&line);
        let my_numbers = get_my_numbers(line);

        let mut wins = 0;
        for my_number in my_numbers {
            for winning_number in winning_numbers.iter() {
                if my_number == *winning_number {
                    wins += 1; 
                }
            }
        }

        wins
    }).collect::<Vec<isize>>();

    let mut total_cards = vec![1; wins_per_card.len()];
    for (index, wins) in wins_per_card.iter().enumerate() {
        let current_total = total_cards[index];
        for _ in 0..current_total as usize {
            for n in index + 1..(index + 1 + *wins as usize) {
                total_cards[n] += 1;
            }
        }
    }

    let sum = total_cards.iter().fold(0, |sum, total| sum + total);

    sum
}

mod tests {
    #[test]
    fn part_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = super::process(input);
        assert_eq!(result, 30);
    }
}
