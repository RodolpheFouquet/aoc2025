mod day6 {
    #[derive(Debug, PartialEq)]
    pub enum Operation {
        Add,
        Multiply,
    }

    impl From<&str> for Operation {
        fn from(str: &str) -> Self {
            if str.trim() == "+" {
                Operation::Add
            } else {
                Operation::Multiply
            }
        }
    }

    #[derive(Debug)]
    pub struct Line {
        pub operation: Operation,
        pub numbers: Vec<u64>,
    }

    impl Line {
        pub fn calc_part1(&self) -> u64 {
            let init = match self.operation {
                Operation::Add => 0,
                Operation::Multiply => 1,
            };

            self.numbers.iter().fold(init, |mut res, x| {
                match self.operation {
                    Operation::Add => res += x,
                    Operation::Multiply => res *= x,
                };

                res
            })
        }
    }

    pub fn get_lines(input: String) -> Vec<Line> {
        let lines: Vec<&str> = input.lines().collect();
        let num_lines = lines.len();
        let mut ret: Vec<_> = lines[num_lines - 1]
            .split_whitespace()
            .map(|operation| Line {
                operation: Operation::from(operation),
                numbers: vec![],
            })
            .collect();
        for i in 0..num_lines - 1 {
            let line: Vec<_> = lines[i].split_whitespace().collect();
            line.into_iter()
                .enumerate()
                .for_each(|(pos, x)| ret[pos].numbers.push(x.parse::<u64>().unwrap()));
        }
        ret
    }

    pub fn get_lines_part2(input: String) -> Vec<Line> {
        let lines: Vec<&str> = input.lines().collect();
        let num_lines = lines.len();
        let mut ret: Vec<_> = lines[num_lines - 1]
            .split_whitespace()
            .map(|operation| Line {
                operation: Operation::from(operation),
                numbers: vec![],
            })
            .collect();
        let lines: Vec<_> = lines
            .into_iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let offsets = find_right_digit_col(&lines[lines.len() - 1]);
        let subgrids = build_cols(&lines, &offsets);
        for i in 0..subgrids.len() {
            ret[i].numbers = build_numbers(&subgrids[i]);
        }
        ret
    }

    pub fn find_right_digit_col(str: &Vec<char>) -> Vec<usize> {
        let mut ret = Vec::new();

        for i in 1..str.len() {
            if str[i] != ' ' {
                ret.push(i - 2);
            }
        }
        ret.push(str.len() - 1);
        ret
    }

    pub type Subgrid = Vec<Vec<u64>>;

    pub fn init_subgrid(width: usize, height: usize) -> Subgrid {
        vec![vec![0; width]; height]
    }

    pub fn build_cols(lines: &Vec<Vec<char>>, last_cols: &Vec<usize>) -> Vec<Subgrid> {
        let mut ret = Vec::new();

        let mut start_col = 0;
        let height = lines.len() - 1;

        for &last_col in last_cols {
            let width = last_col - start_col + 1;
            let mut subgrid = init_subgrid(width, height);
            for i in 0..width {
                for j in 0..height {
                    subgrid[j][i] = if lines[j][start_col + i] == ' ' {
                        0
                    } else {
                        lines[j][start_col + i].to_string().parse::<u64>().unwrap()
                    }
                }
            }
            start_col = last_col + 2;

            ret.push(subgrid);
        }
        ret
    }

    pub fn build_numbers(subgrid: &Subgrid) -> Vec<u64> {
        let mut ret = Vec::new();
        let lines = subgrid.len();
        let width = subgrid[0].len();
        for i in (0..width).rev() {
            let mut number: u64 = 0;
            for j in 0..lines {
                if subgrid[j][i] != 0 {
                    number = number * 10 + subgrid[j][i];
                }
            }
            ret.push(number);
        }
        ret
    }

    pub fn part1(input: String) -> u64 {
        get_lines(input)
            .into_iter()
            .map(|line| line.calc_part1())
            .fold(0, |mut sum, x| {
                sum += x;
                sum
            })
    }

    pub fn part2(input: String) -> u64 {
        get_lines_part2(input)
            .into_iter()
            .map(|line| line.calc_part1())
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
    fn test_operation() {
        assert_eq!(day6::Operation::from("+"), day6::Operation::Add);
    }

    #[test]
    fn test_build_bols() {
        let file = fs::read_to_string("assets/day6_test.txt").unwrap();
        let lines = file
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let offsets = day6::find_right_digit_col(&lines[lines.len() - 1]);
        let subgrids = day6::build_cols(&lines, &offsets);
        assert_eq!(offsets, vec![2, 6, 10, 14]);
        assert_eq!(
            subgrids,
            [
                [[1, 2, 3], [0, 4, 5], [0, 0, 6]],
                [[3, 2, 8], [6, 4, 0], [9, 8, 0]],
                [[0, 5, 1], [3, 8, 7], [2, 1, 5]],
                [[6, 4, 0], [2, 3, 0], [3, 1, 4]]
            ]
        );

        let numbers: Vec<_> = subgrids.iter().map(|s| day6::build_numbers(s)).collect();
        assert_eq!(
            numbers,
            [[356, 24, 1], [8, 248, 369], [175, 581, 32], [4, 431, 623]]
        );
    }

    #[test]
    fn test_line() {
        assert_eq!(
            day6::Line {
                operation: day6::Operation::Add,
                numbers: vec![1, 3, 4]
            }
            .calc_part1(),
            8
        );
        assert_eq!(
            day6::Line {
                operation: day6::Operation::Multiply,
                numbers: vec![1, 3, 4]
            }
            .calc_part1(),
            12
        );
    }

    #[test]
    fn test_part1() {
        let file = fs::read_to_string("assets/day6_test.txt").unwrap();
        assert_eq!(day6::part1(file), 4277556);
        let file = fs::read_to_string("assets/day6.txt").unwrap();
        assert_eq!(day6::part1(file), 6343365546996);
    }

    #[test]
    fn test_last_op() {
        let file = fs::read_to_string("assets/day6_test.txt").unwrap();
        let last_line = file
            .lines()
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .chars()
            .collect::<Vec<char>>();
        assert_eq!(day6::find_right_digit_col(&last_line), vec![2, 6, 10, 14]);
    }

    #[test]
    fn test_part2() {
        let file = fs::read_to_string("assets/day6_test.txt").unwrap();
        assert_eq!(day6::part2(file), 3263827);
        let file = fs::read_to_string("assets/day6.txt").unwrap();
        assert_eq!(day6::part2(file), 11136895955912);
    }
}
