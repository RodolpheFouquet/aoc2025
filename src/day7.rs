mod day7 {
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Cell {
        Beam,
        Empty,
        Source,
        Splitter,
    }

    impl From<char> for Cell {
        fn from(str: char) -> Self {
            match str {
                '|' => Cell::Beam,
                'S' => Cell::Source,
                '^' => Cell::Splitter,
                _ => Cell::Empty,
            }
        }
    }

    impl Into<char> for Cell {
        fn into(self) -> char {
            match self {
                Cell::Empty => '.',
                Cell::Source => 'S',
                Cell::Beam => '|',
                Cell::Splitter => '^',
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Grid {
        pub width: usize,
        pub height: usize,
        pub data: Vec<Cell>,
    }

    impl Into<String> for Grid {
        fn into(self) -> String {
            let mut ret: String = "".to_owned();
            for j in 0..self.height - 1 {
                for i in 0..self.width {
                    ret.push(self.data[j * self.width + i].into());
                }
                ret.push('\n');
            }
            for i in 0..self.width {
                ret.push(self.data[(self.height - 1) * self.width + i].into());
            }
            ret
        }
    }

    impl From<&str> for Grid {
        fn from(str: &str) -> Self {
            let lines: Vec<_> = str.lines().collect();
            let width = lines[0].len();
            let height = lines.len();

            Self {
                width,
                height,
                data: lines
                    .iter()
                    .flat_map(|&line| line.chars().map(|c| Cell::from(c)).collect::<Vec<Cell>>())
                    .collect::<Vec<Cell>>(),
            }
        }
    }

    pub fn part1(input: String) -> u64 {
        let mut grid = Grid::from(input.as_str());
        let mut split_count = 0;
        for j in 0..grid.height - 1 {
            // println!("processing line {}", j);
            // find source or beams below
            for i in 0..grid.width {
                if grid.data[j * grid.width + i] == Cell::Source {
                    grid.data[(j + 1) * grid.width + i] = Cell::Beam
                } else if grid.data[j * grid.width + i] == Cell::Beam
                    && grid.data[(j + 1) * grid.width + i] == Cell::Splitter
                {
                    split_count += 1;

                    if i > 0 {
                        grid.data[(j + 1) * grid.width + i - 1] = Cell::Beam
                    }
                    if i < grid.width - 1 {
                        grid.data[(j + 1) * grid.width + i + 1] = Cell::Beam
                    }
                } else if grid.data[j * grid.width + i] == Cell::Beam
                    && grid.data[(j + 1) * grid.width + i] == Cell::Empty
                {
                    grid.data[(j + 1) * grid.width + i] = Cell::Beam
                }
            }
            // let s: String = grid.clone().into();
            // println!("{}", s);
        }
        split_count
    }

    pub fn part2(input: String) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_grid() {
        let input = "|^\n.S";
        assert_eq!(
            day7::Grid::from(input),
            day7::Grid {
                width: 2,
                height: 2,
                data: vec![
                    day7::Cell::Beam,
                    day7::Cell::Splitter,
                    day7::Cell::Empty,
                    day7::Cell::Source,
                ]
            }
        );
        let grid = day7::Grid::from(input);
        let str: String = grid.into();
        assert_eq!(str, input);
    }

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("assets/day7_test.txt")
            .expect("day7_test.txt should have been read");

        assert_eq!(day7::part1(input.into()), 21);
        let file = fs::read_to_string("assets/day7.txt").expect("day7.txt should have been read");
        assert_eq!(day7::part1(file), 1598);
    }

    #[test]
    fn test_part2() {
        // let input =
        // fs::read_to_string("assets/day1_test.txt").expect("day1.txt should have been read");
        // assert_eq!(day1::part2(input.into()), Ok(6));
        // let file = fs::read_to_string("assets/day1.txt").expect("day1.txt should have been read");
        // assert_eq!(day1::part2(file), Ok(6106));
    }
}
