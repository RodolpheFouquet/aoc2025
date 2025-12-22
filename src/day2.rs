mod day2 {

    pub fn invalid(number: &str) -> bool {
        if number.len() % 2 == 1 {
            false
        } else {
            let half = number.len() / 2;

            let first_half = &number[0..half];
            let second_half = &number[half..];
            String::from(first_half) == String::from(second_half)
        }
    }

    pub fn find_all_invalid(begin: u64, end: u64) -> Vec<u64> {
        let mut result = Vec::new();
        for i in begin..=end {
            if invalid(&i.to_string()) {
                result.push(i);
            }
        }
        result
    }

    pub fn invalid_p2(number: &str) -> bool {
        let mid = number.len() / 2;
        for i in 1..=mid {
            let potential_pattern = &number[0..i];
            if number.len() % potential_pattern.len() != 0 {
                continue;
            }
            let mut j = i;
            let mut broke = false;
            while j < number.len() {
                let next = &number[j..j + potential_pattern.len()];
                if next != potential_pattern {
                    broke = true;
                    break;
                }
                j += potential_pattern.len();
            }

            if !broke {
                return true;
            }
        }
        false
    }

    pub fn find_all_invalid_p2(begin: u64, end: u64) -> Vec<u64> {
        let mut result = Vec::new();
        for i in begin..=end {
            if invalid_p2(&i.to_string()) {
                result.push(i);
            }
        }
        result
    }

    pub fn parse_interval(input: &str) -> (u64, u64) {
        let parts: Vec<&str> = input.trim().split("-").collect();

        (
            parts[0].parse::<u64>().unwrap(),
            parts[1].parse::<u64>().unwrap(),
        )
    }
    pub fn part1(input: String) -> u64 {
        let intervals: Vec<(u64, u64)> = input
            .split(",")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(parse_interval)
            .collect();

        intervals
            .iter()
            .flat_map(|interval| find_all_invalid(interval.0, interval.1))
            .fold(0, |mut sum, x| {
                sum += x;
                sum
            })
    }

    pub fn part2(input: String) -> u64 {
        let intervals: Vec<(u64, u64)> = input
            .split(",")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(parse_interval)
            .collect();
        intervals
            .iter()
            .flat_map(|interval| find_all_invalid_p2(interval.0, interval.1))
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
    fn test_invalid() {
        assert!(!day2::invalid("1".into()));
        assert!(day2::invalid("11".into()));
        assert!(day2::invalid("99".into()));
        assert!(day2::invalid("2222".into()));
        assert!(day2::invalid("1188511885".into()));
        assert!(day2::invalid("38593859".into()));
    }

    #[test]
    fn test_invalid_p2() {
        assert!(!day2::invalid_p2("1".into()));
        assert!(day2::invalid_p2("11".into()));
        assert!(day2::invalid_p2("99".into()));
        assert!(day2::invalid_p2("2222".into()));
        assert!(day2::invalid_p2("1188511885".into()));
        assert!(day2::invalid_p2("38593859".into()));
        assert!(day2::invalid_p2("824824824".into()));
        assert!(day2::invalid_p2("2121212121".into()));
        assert!(!day2::invalid_p2("101".into()));
        assert!(day2::invalid_p2("111".into()));
        assert!(!day2::invalid_p2("112".into()));
    }

    #[test]
    fn test_find_invalid() {
        assert_eq!(day2::find_all_invalid(11, 22), vec![11, 22]);
        assert_eq!(day2::find_all_invalid(95, 115), vec![99]);
        assert_eq!(day2::find_all_invalid(998, 1012), vec![1010]);
        assert_eq!(
            day2::find_all_invalid(1188511880, 1188511890),
            vec![1188511885]
        );
    }

    #[test]
    fn test_find_invalid_p2() {
        assert_eq!(day2::find_all_invalid_p2(11, 22), vec![11, 22]);
        assert_eq!(day2::find_all_invalid_p2(95, 115), vec![99, 111]);
        assert_eq!(day2::find_all_invalid_p2(998, 1012), vec![999, 1010]);
        assert_eq!(
            day2::find_all_invalid_p2(1188511880, 1188511890),
            vec![1188511885]
        );
    }

    #[test]
    fn test_parse_interval() {
        assert_eq!(day2::parse_interval("11-22"), (11, 22));
        assert_eq!(day2::parse_interval("11-223"), (11, 223));
    }

    #[test]
    fn test_part1() {
        let file = fs::read_to_string("assets/day2_test.txt").unwrap();
        assert_eq!(day2::part1(file.into()), 1227775554);
        let file = fs::read_to_string("assets/day2.txt").unwrap();
        assert_eq!(day2::part1(file.into()), 13919717792);
    }

    #[test]
    fn test_part2() {
        let file = fs::read_to_string("assets/day2_test.txt").unwrap();
        assert_eq!(day2::part2(file.into()), 4174379265);
        let file = fs::read_to_string("assets/day2.txt").unwrap();
        assert_eq!(day2::part2(file.into()), 14582313461);
    }
}
