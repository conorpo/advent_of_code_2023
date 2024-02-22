



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown
}

struct Row {
    cells: Vec<SpringState>,
    damaged_contiguous_pattern: Vec<usize>,
}

impl Row {
    fn get_unknown_vec(&self) -> Vec<usize> {
        self.cells.iter().enumerate().filter_map(|(i, &s)| if s == SpringState::Unknown { Some(i) } else { None }).collect()
    }

    fn dynamic_solver(&mut self) ->  usize{
        let unknown = self.get_unknown_vec();
        let damaged_pattern = self.damaged_contiguous_pattern.clone();

        let w = self.cells.len() + 1;
        let h = damaged_pattern.iter().sum::<usize>() + damaged_pattern.len() + 1;
        let mut dp = vec![vec![0; w]; h];
        
        // The rows of the dp represent where we are in the damaged pattern
        // For example for the pattern [1,1,3] the rows represent
        /*
        0: Operational before first damaged streak
        1: First damaged streak, spring #1
        2: Operational between first and second damaged streak
        3: Second damaged streak, spring #1
        4: Operational between second and third damaged streak
        5: Third damaged streak, spring #1
        6: Third damaged streak, spring #2
        7: Third damaged streak, spring #3
        8: Operational after third damaged streak
        */

        dp[0][0] = 1; // We start with 1 operational spring and 0 damaged streaks

        for x in 1..w {
            dp[0][x] = if let SpringState::Damaged = self.cells[x - 1] {
                0
            } else {
                dp[0][x-1]
            };
        }


        for x in 1..w {
            let mut y = 0;  
            for y_1 in damaged_pattern.iter() {
                for _ in 0..*y_1 {
                    y += 1;

                    // x=1, y=1 means we are on the first cell and we are expecting to be done with the first spring of the first pattern
                    dp[y][x] = if let SpringState::Operational = self.cells[x - 1] {
                        // Unexpected Operational
                        0
                    } else {
                        // Continue damaged streak or just started
                        dp[y-1][x-1]
                    };
                }

                y += 1;

                // Check if we are done with the damaged streak
                dp[y][x] = if let SpringState::Damaged = self.cells[x - 1] {
                    // Damage streak too long
                    0
                } else {
                    // Operational streak
                    dp[y-1][x-1] + dp[y][x-1]
                };
            }
        }

        //dbg!(&dp);

        dp[h - 1][w - 1]
    }
}

fn main() {
    let input = include_str!("./12.txt");
    let part1_output = part1(input);
    dbg!(part1_output);
    let part2_output = part2(input);
    dbg!(part2_output);
}

fn part1(input: &str) -> usize {
    let mut sum: usize = 0;

    for line in input.lines() {
        let (springs, pattern) = line.split_once(' ').unwrap();

        let mut row = Row {
            cells: springs.chars().map(|c| match c {
                '?' => SpringState::Unknown,
                '.' => SpringState::Operational,
                '#' => SpringState::Damaged,
                _ => unreachable!()
            }).collect(),
            damaged_contiguous_pattern: pattern.split(',').map(|s| s.parse().unwrap()).collect()
        };

        row.cells.push(SpringState::Operational);

        let result = row.dynamic_solver();

        //dbg!(result);

        sum += result;
    }

    sum
}

fn part2(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let (springs, pattern) = line.split_once(' ').unwrap();

        // Copy springs 5 times
        let mut springs_vec: Vec<SpringState> = springs.chars().map(|c| match c {
            '?' => SpringState::Unknown,
            '.' => SpringState::Operational,
            '#' => SpringState::Damaged,
            _ => unreachable!()
        }).collect();
        springs_vec.push(SpringState::Unknown);

        let springs_vec: Vec<SpringState> = springs_vec.iter().cloned().cycle().take((springs.len() + 1) * 5 - 1).collect();

        // Copy pattern 5 times
        let pattern_vec: Vec<usize> = pattern.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        let len = pattern_vec.len();
        let pattern_vec: Vec<usize> = pattern_vec.iter().cloned().cycle().take(len * 5).collect();

        let mut row = Row {
            cells: springs_vec,
            damaged_contiguous_pattern: pattern_vec
        };

        row.cells.push(SpringState::Operational);

        let result = row.dynamic_solver();

        //dbg!(result);

        sum += result;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const test_input: &str = 
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input), 0);
    }
}
