mod day4 {

    pub fn count_adjacent(x: usize, y: usize, grid: &Vec<Vec<char>>) -> u64 {
        let height = grid.len() as isize;
        let width = grid[0].len() as isize;

        let offsets = vec![
            (1, 0),
            (0, 1),
            (1, 1),
            (-1, 1),
            (-1, 0),
            (0, -1),
            (-1, -1),
            (1, -1),
        ];

        let sum = offsets
            .into_iter()
            .map(|(offset_x, offset_y)| (x as isize - offset_x, y as isize - offset_y))
            .map(|(x, y)| {
                if x < 0 || y < 0 || x > width - 1 || y > height - 1 {
                    0
                } else if grid[y as usize][x as usize] == '.' {
                    0
                } else {
                    1
                }
            })
            .fold(0, |mut sum, x| {
                sum += x;
                sum
            });
        sum
    }

    pub fn part1(input: String) -> u64 {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let height = grid.len();
        let width = grid[0].len();
        let mut sum = 0;
        for y in 0..height {
            for x in 0..width {
                let count = count_adjacent(x, y, &grid);
                if grid[y][x] == '@' {
                    sum += if count < 4 { 1 } else { 0 }
                }
            }
        }
        sum
    }

    pub fn part2(input: String) -> u64 {
        let mut grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut sum = 0;
        let height = grid.len();
        let width = grid[0].len();

        loop {
            let mut cur_sum = 0;
            let mut positions_to_clear = vec![];
            for y in 0..height {
                for x in 0..width {
                    let count = count_adjacent(x, y, &grid);
                    if grid[y][x] == '@' {
                        cur_sum += if count < 4 { 1 } else { 0 }
                    }
                    if count < 4 {
                        positions_to_clear.push((x, y));
                    }
                }
            }

            for (x, y) in positions_to_clear {
                grid[y][x] = '.';
            }
            if cur_sum == 0 {
                break;
            }
            sum += cur_sum;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let file = fs::read_to_string("assets/day4_test.txt").unwrap();
        assert_eq!(day4::part1(file), 13);
        let file = fs::read_to_string("assets/day4.txt").unwrap();
        assert_eq!(day4::part1(file), 1493);
    }

    #[test]
    fn test_part2() {
        let file = fs::read_to_string("assets/day4_test.txt").unwrap();
        assert_eq!(day4::part2(file), 43);
        let file = fs::read_to_string("assets/day4.txt").unwrap();
        assert_eq!(day4::part2(file), 9194);
    }
}
