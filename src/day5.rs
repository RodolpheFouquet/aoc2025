mod day5 {
    use std::cmp;
    use std::cmp::Ordering;

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct Interval {
        pub min: u64,
        pub max: u64,
    }
    impl Interval {
        fn overlap(&self, other: &Self) -> bool {
            self.min <= other.max && other.min <= self.max
        }

        fn merge(&self, other: &Self) -> Self {
            Self {
                min: cmp::min(self.min, other.min),
                max: cmp::max(self.max, other.max),
            }
        }
    }

    impl From<&str> for Interval {
        fn from(str: &str) -> Self {
            let components: Vec<_> = str.trim().split("-").collect();
            Interval {
                min: components[0].parse::<u64>().unwrap(),
                max: components[1].parse::<u64>().unwrap(),
            }
        }
    }

    pub struct Database {
        pub fresh: Vec<Interval>,
        pub available: Vec<u64>,
    }

    impl PartialOrd for Interval {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.min.cmp(&other.min))
        }
    }
    impl Ord for Interval {
        fn cmp(&self, other: &Self) -> Ordering {
            self.min.cmp(&other.min)
        }
    }
    impl From<&str> for Database {
        fn from(str: &str) -> Self {
            let parts: Vec<_> = str.split("\n\n").collect();

            let mut fresh = parts[0]
                .lines()
                .map(|line| Interval::from(line))
                .collect::<Vec<_>>();
            fresh.sort();

            let mut compressed = Vec::new();
            let mut current = fresh[0];
            for i in 1..fresh.len() {
                if current.overlap(&fresh[i]) {
                    current = current.merge(&fresh[i]);
                } else {
                    compressed.push(current);
                    current = fresh[i].clone();
                }
            }

            compressed.push(current);
            let available = parts[1]
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .collect();
            Database {
                fresh: compressed,
                available: available,
            }
        }
    }

    impl Database {
        fn count_part1(&self) -> u64 {
            self.available
                .iter()
                .map(|val| {
                    self.fresh
                        .iter()
                        .any(|interval| interval.min <= *val && interval.max >= *val)
                })
                .fold(0, |mut sum, x| {
                    if x {
                        sum += 1;
                        sum
                    } else {
                        sum
                    }
                })
        }

        fn count_part2(&self) -> u64 {
            self.fresh
                .iter()
                .map(|interval| interval.max - interval.min + 1)
                .fold(0, |mut sum, x| {
                    sum += x;
                    sum
                })
        }
    }
    pub fn part1(input: String) -> u64 {
        let database = Database::from(input.as_str());
        database.count_part1()
    }

    pub fn part2(input: String) -> u64 {
        let database = Database::from(input.as_str());
        database.count_part2()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_interval() {
        assert_eq!(
            day5::Interval::from("1-17"),
            day5::Interval { min: 1, max: 17 }
        )
    }

    #[test]
    fn test_parse_database() {
        let file = fs::read_to_string("assets/day5_test.txt").unwrap();
        assert_eq!(
            day5::Database::from(file.as_str()).fresh,
            vec![
                day5::Interval { min: 3, max: 5 },
                day5::Interval { min: 10, max: 20 },
            ],
        );
        assert_eq!(
            day5::Database::from(file.as_str()).available,
            vec![1, 5, 8, 11, 17, 32]
        );
    }

    #[test]
    fn test_part1() {
        let file = fs::read_to_string("assets/day5_test.txt").unwrap();
        assert_eq!(day5::part1(file), 3);
        let file = fs::read_to_string("assets/day5.txt").unwrap();
        assert_eq!(day5::part1(file), 739);
    }

    #[test]
    fn test_part2() {
        let file = fs::read_to_string("assets/day5_test.txt").unwrap();
        assert_eq!(day5::part2(file), 14);
        let file = fs::read_to_string("assets/day5.txt").unwrap();
        assert_eq!(day5::part2(file), 344486348901788);
    }
}
