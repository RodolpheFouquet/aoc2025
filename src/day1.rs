mod day1 {
    use std::str::FromStr;
    #[derive(Debug, PartialEq)]
    pub enum Move {
        Right(i64),
        Left(i64),
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct ParseMoveError;

    impl FromStr for Move {
        type Err = ParseMoveError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let input = s.trim();

            let direction = input.chars().nth(0).ok_or(ParseMoveError);

            let number = s[1..].parse::<i64>().map_err(|_| ParseMoveError)?;
            match direction {
                Ok(d) if d == 'L' => Ok(Move::Left(number)),
                Ok(d) if d == 'R' => Ok(Move::Right(number)),
                _ => Err(ParseMoveError),
            }
        }
    }

    pub fn part1(input: String) -> Result<u64, ParseMoveError> {
        //let file = fs::read_to_string(input).expect("day1.txt should have been read");
        let moves: Result<Vec<_>, ParseMoveError> =
            input.lines().map(|line| Move::from_str(line)).collect();
        let mut counter = 0;
        let mut start: i64 = 50;
        match moves {
            Ok(mvs) => {
                mvs.iter().for_each(|m| {
                    match m {
                        Move::Left(u) => {
                            for _ in 0..*u {
                                start = (start - 1 + 100) % 100;
                            }
                        }
                        Move::Right(u) => {
                            for _ in 0..*u {
                                start = (start + 1) % 100;
                            }
                        }
                    };
                    if start == 0 {
                        counter += 1;
                    }
                });
                Ok(counter)
            }
            _ => Err(ParseMoveError),
        }
    }

    pub fn part2(input: String) -> Result<i64, ParseMoveError> {
        //let file = fs::read_to_string(input).expect("day1.txt should have been read");
        let moves: Result<Vec<_>, ParseMoveError> =
            input.lines().map(|line| Move::from_str(line)).collect();
        let mut counter = 0;
        let mut start: i64 = 50;
        match moves {
            Ok(mvs) => {
                mvs.iter().for_each(|m| {
                    match m {
                        Move::Left(u) => {
                            for _ in 0..*u {
                                start = (start - 1 + 100) % 100;
                                if start == 0 {
                                    counter += 1;
                                }
                            }
                        }
                        Move::Right(u) => {
                            for _ in 0..*u {
                                start = (start + 1) % 100;
                                if start == 0 {
                                    counter += 1;
                                }
                            }
                        }
                    };
                });
                Ok(counter)
            }
            _ => Err(ParseMoveError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::str::FromStr;

    #[test]
    fn test_move() {
        let m: Result<day1::Move, day1::ParseMoveError> = day1::Move::from_str("L10");
        assert_eq!(m, Ok(day1::Move::Left(10)));
        let m: Result<day1::Move, day1::ParseMoveError> = day1::Move::from_str("R5");
        assert_eq!(m, Ok(day1::Move::Right(5)));
        let m: Result<day1::Move, day1::ParseMoveError> = day1::Move::from_str("R");
        assert_eq!(m, Err(day1::ParseMoveError));
    }

    #[test]
    fn test_part1() {
        let input =
            fs::read_to_string("assets/day1_test.txt").expect("day1.txt should have been read");
        assert_eq!(day1::part1(input.into()), Ok(3));
        let file = fs::read_to_string("assets/day1.txt").expect("day1.txt should have been read");
        assert_eq!(day1::part1(file), Ok(982));
    }

    #[test]
    fn test_part2() {
        let input =
            fs::read_to_string("assets/day1_test.txt").expect("day1.txt should have been read");
        assert_eq!(day1::part2(input.into()), Ok(6));
        let file = fs::read_to_string("assets/day1.txt").expect("day1.txt should have been read");
        assert_eq!(day1::part2(file), Ok(6106));
    }
}
