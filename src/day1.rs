pub fn solution(input: Vec<&str>) -> u32 {
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
