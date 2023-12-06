pub fn read_input() -> Vec<&'static str> {
    include_str!("day1_input1.txt").lines().collect()
}

pub fn solution_one(input: Vec<&str>) -> u32 {
    let mut sum = 0;
    for line in input {
        let digits: Vec<u32> = line
            .chars()
            .filter(|char| char.is_ascii_digit())
            .map(|char| char.to_digit(10).unwrap())
            .collect();
        sum += 10*digits[0] + digits[digits.len()-1];
    }
    return sum;
}

pub fn solution_two(input: Vec<&str>) -> u32 {
    // zero one two three four five seven eight nine
    return 142;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn first_part() {
        let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        assert_eq!(solution_one(input), 142);
    }

    #[test]
    fn second_part() {
        let input = vec!["two1nine", "eightwothree", "abcone2threexyz", "xtwone3four", "4nineeightseven2", "zoneight234", "7pqrstsixteen"];
        assert_eq!(solution_two(input), 281);
    }
}
