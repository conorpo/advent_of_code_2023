fn main() {
    let input = include_str!("./13.txt");
    let part1_output = part1(input);
    dbg!(part1_output);
    let part2_output = part2(input);
    dbg!(part2_output);
}

fn part1(input: &str) -> u32 {
    let patterns = input.split("\r\n\r\n");

    let mut sum:u32 = 0;
    for pattern in patterns {

        let cols = pattern.lines().next().unwrap().chars().count();

        let mut row_bitmask: Vec<u32> = vec![];
        let mut col_bitmask: Vec<u32> = vec![0; cols];

        let mut lines = pattern.lines();
        for (i, line) in lines.enumerate() {
            //dbg!(line);
            row_bitmask.push(0);
            let row_bit = 1 << i;

            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    row_bitmask[i] |= 1 << j;
                    col_bitmask[j] |= row_bit;
                }
            }
        }

        let mut possible_horizontal_mirror_spots: Vec<usize> = vec![];
        let mut possible_vertical_mirror_spots: Vec<usize> = vec![];

        for i in 1..row_bitmask.len() {
            if row_bitmask[i] == row_bitmask[i-1] {
                possible_horizontal_mirror_spots.push(i);
            }
        }

        for i in 1..col_bitmask.len() {
            if col_bitmask[i] == col_bitmask[i-1] {
                possible_vertical_mirror_spots.push(i);
            }
        }

        let mut mirrors = 0;

        while let Some(i) = possible_horizontal_mirror_spots.pop() {
            let mut valid = true;

            let mut t = i - 1;
            let mut b = i;

            while valid && t > 0 && b < row_bitmask.len() - 1 {
                t -= 1;
                b += 1;

                valid = row_bitmask[t] == row_bitmask[b];
            }

            if valid {
                //dbg!(i, row_bitmask.len());
                sum += 100 * i as u32;
                mirrors += 1;
            }
        }

        while let Some(i) = possible_vertical_mirror_spots.pop() {
            let mut valid = true;

            let mut l = i - 1;
            let mut r = i;

            while valid && l > 0 && r < col_bitmask.len() - 1 {
                l -= 1;
                r += 1;

                valid = col_bitmask[l] == col_bitmask[r];
            }

            if valid {
                sum += (i) as u32;
                mirrors += 1;
                print!("Vertical mirror at {}, adding {}\n", i, i);
            }
        }

        assert_eq!(mirrors, 1)

    }

    sum
}

fn part2(input: &str) -> u32 {
    let patterns = input.split("\r\n\r\n");

    let mut sum:u32 = 0;
    for pattern in patterns {
        let cols = pattern.lines().next().unwrap().chars().count();

        let mut row_bitmask: Vec<u32> = vec![];
        let mut col_bitmask: Vec<u32> = vec![0; cols];

        let mut lines = pattern.lines();
        for (i, line) in lines.enumerate() {
            //dbg!(line);
            row_bitmask.push(0);
            let row_bit = 1 << i;

            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    row_bitmask[i] |= 1 << j;
                    col_bitmask[j] |= row_bit;
                }
            }
        }

        //Incase our mirror spot already contains the smudge, we attach the difcount to the potential spots
        let mut possible_horizontal_mirror_spots: Vec<(usize,u32)> = vec![];
        let mut possible_vertical_mirror_spots: Vec<(usize,u32)> = vec![];

        for i in 1..row_bitmask.len() {
            let dif = (row_bitmask[i] ^ row_bitmask[i-1]).count_ones();
            if dif <= 1 {
                possible_horizontal_mirror_spots.push((i, dif));
            }
        }

        for i in 1..col_bitmask.len() {
            let dif = (col_bitmask[i] ^ col_bitmask[i-1]).count_ones();
            if dif <= 1 {
                possible_vertical_mirror_spots.push((i, dif));
            }
        }

        let mut mirrors = 0;

        while let Some((i, dif)) = possible_horizontal_mirror_spots.pop() {
            let mut t = i - 1;
            let mut b = i;
            let mut dif = dif;

            // Now we check until the amount of changes it would require is greater than 1
            while dif <= 1 && t > 0 && b < row_bitmask.len() - 1 {
                t -= 1;
                b += 1;

                dif += (row_bitmask[t] ^ row_bitmask[b]).count_ones();
            }

            //Mirror spot that would exist if we fixed some smudge
            if dif == 1 {
                sum += 100 * i as u32;
                mirrors += 1;
            }
        }

        while let Some((i, dif)) = possible_vertical_mirror_spots.pop() {
            let mut l = i - 1;
            let mut r = i;
            let mut dif = dif;

            while dif <= 1 && l > 0 && r < col_bitmask.len() - 1 {
                l -= 1;
                r += 1;

                dif += (col_bitmask[l] ^ col_bitmask[r]).count_ones();
            }

            if dif == 1 {
                sum += (i) as u32;
                mirrors += 1;
            }
        }

        assert_eq!(mirrors, 1)
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const test_input: &str =
    "#.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#";
    
    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input), 0);
    }
}
