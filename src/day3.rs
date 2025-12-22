mod day3 {

    pub fn highest_joltage(numbers: Vec<u64>) -> u64 {
        let mut max = 0;
        for i in 0..numbers.len() - 1 {
            for j in i + 1..numbers.len() {
                if max < numbers[i] * 10 + numbers[j] {
                    max = numbers[i] * 10 + numbers[j]
                }
            }
        }
        max
    }

    pub fn highest_joltage2(numbers: Vec<u64>) -> u64 {
        let max_len = 12;
        let len = numbers.len();

        let mut stack: Vec<u64> = vec![];
        numbers.into_iter().enumerate().for_each(|(pos, x)| {
            while stack.len() > 0 && *stack.last().unwrap() < x && stack.len() + len - pos > max_len
            {
                stack.pop();
            }
            if stack.len() < max_len {
                stack.push(x);
            }
        });

        stack
            .into_iter()
            .rev()
            .enumerate()
            .fold(0, |mut sum, (pos, x)| {
                sum = sum + 10_u64.pow((pos as u64).try_into().unwrap()) * x;
                sum
            })
    }

    pub fn to_numbers(input: &str) -> Vec<u64> {
        input
            .chars()
            .map(|x| x.to_string().parse::<u64>().unwrap())
            .collect()
    }
    pub fn part1(input: String) -> u64 {
        input
            .lines()
            .map(|line| highest_joltage(to_numbers(line)))
            .fold(0, |mut sum, x| {
                sum += x;
                sum
            })
    }

    pub fn part2(input: String) -> u64 {
        input
            .lines()
            .map(|line| highest_joltage2(to_numbers(line)))
            .fold(0, |mut sum, x| {
                sum += x;
                sum
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_highest_joltage() {
        assert_eq!(
            day3::highest_joltage(day3::to_numbers("987654321111111")),
            98
        );
        assert_eq!(
            day3::highest_joltage(day3::to_numbers("811111111111119")),
            89
        );
        assert_eq!(
            day3::highest_joltage(day3::to_numbers("234234234234278")),
            78
        );
        assert_eq!(
            day3::highest_joltage(day3::to_numbers("818181911112111")),
            92
        );
    }

    #[test]
    fn test_highest_joltage_2() {
        assert_eq!(
            day3::highest_joltage2(day3::to_numbers("811111111111119")),
            811111111119
        );
        assert_eq!(
            day3::highest_joltage2(day3::to_numbers("234234234234278")),
            434234234278
        );
    }

    #[test]
    fn test_part1() {
        let file = fs::read_to_string("assets/day3_test.txt").unwrap();
        assert_eq!(day3::part1(file), 357);
        let file = fs::read_to_string("assets/day3.txt").unwrap();
        assert_eq!(day3::part1(file), 17412);
    }

    #[test]
    fn test_part2() {
        let file = fs::read_to_string("assets/day3_test.txt").unwrap();
        assert_eq!(day3::part2(file), 3121910778619);
        let file = fs::read_to_string("assets/day3.txt").unwrap();
        assert_eq!(day3::part2(file), 172681562473501);
    }
}
