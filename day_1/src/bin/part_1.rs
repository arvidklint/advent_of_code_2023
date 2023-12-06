fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> isize {
    let mut sum = 0;
    for line in input.lines() {
        let mut min = '0';
        let mut found_min = false;
        let mut max = '0';
        for character in line.chars() {
            if character.is_numeric() {
                if !found_min {
                    min = character;
                    found_min = true;
                }
                max = character;
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
    fn part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = super::part_1(input);
        assert_eq!(result, 142);
    }
}