/*
Day 3 the problem itself wasn't too bad, but dealing with iteration / incrementing in standard rust was tough. 
Not gonna use any crates tho as I wanna get better at the base language.
These types of algorithims were cells need to check adjacent cells (or maybe even flood fill out) aren't great with iterators,
If I redid this I'd probably just s

*/


//Puzzle Input


fn main() {
    let input = include_str!("./3.txt");
    let output = part1(input);
    dbg!(output);
    let output2 = part2(input);
    dbg!(output2);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CellType {
    Empty,
    Num{part: bool, num: u32},
    Symbol,
    Gear{id: i32}
}

// In yx
const ADJACENT_DIRS: [(i32,i32);8] = [(-1,1),(0,1),(1,1),(1,0),(1,-1),(0,-1),(-1,-1),(-1,0)];

// Creates a grid with padding, returns (grid, width, height)
fn create_grid(input: &str, handle_gears: bool) -> (Vec<Vec<CellType>>, usize, usize, i32){
    let mut size_finder = input.lines();

    let w = size_finder.next().unwrap().chars().count() + 2; // +2 for padding
    let h = size_finder.count() + 3; // +2 for padding, +1 for the first line

    let mut grid = Vec::<Vec<CellType>>::with_capacity(h);

    let mut gear_id: i32 = -1;

    grid.push(vec![CellType::Empty; w]);

    for line in input.lines() {
        let mut row = Vec::<CellType>::with_capacity(w);

        row.push(CellType::Empty);

        for c in line.chars() {
            match c {
                '.' => row.push(CellType::Empty),
                c if c.is_digit(10) => row.push(CellType::Num{ part: false, num: c.to_digit(10).unwrap() }), 
                '*' if handle_gears => row.push(CellType::Gear{id:  {
                                                                        gear_id += 1;
                                                                        gear_id
                                                                    }}),
                _ => row.push(CellType::Symbol)
            }
        }

        row.push(CellType::Empty);

        grid.push(row);
    }

    grid.push(vec![CellType::Empty; w]);

    (grid, w, h, gear_id + 1)
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    
    let (mut grid, w, h, _) = create_grid(input, false);

    // Check symbol adjacency
    for y in 1..h-1 {
        for x in 1..w-1 {
            if grid[y][x] != CellType::Symbol { continue; }

            for dir in ADJACENT_DIRS {
                let nx = ((x as i32) + dir.1) as usize;
                let ny = ((y as i32) + dir.0) as usize;

                if let CellType::Num {ref mut part, num: _ } = grid[ny][nx] {
                    *part = true;
                }
            }
        }
    }

    for line in grid.iter() {
        let mut cell_itr = line.iter();
        while let Some(cell) = cell_itr.next() {
            if let CellType::Num{part, num} = cell {
                let mut num_sum = *num;
                let mut num_is_part = *part;

                while let Some(CellType::Num{part, num}) = cell_itr.next() {
                    num_sum *= 10;
                    num_sum += *num;
                    num_is_part = num_is_part || *part;
                }

                if num_is_part {
                    sum += num_sum;
                }          
            }
        }
    }

    sum
}

fn part2(input: &str) -> u32{
    let (grid, w, h, gear_count) = create_grid(input, true);

    let mut gear_ratios = vec![(1,0) ; gear_count as usize];

    // Check number adjacency
    let mut x = 1;
    let mut y = 1;

    while y < h-1 {
        while x < w-1 {
            if let CellType::Num{part:_, num:_} = grid[y][x] {
                let mut num_sum = 0;
                let mut gears = Vec::<CellType>::new();

                // Check for gears to the left
                for dir in ADJACENT_DIRS[4..7].iter() {
                    let nx = ((x as i32) + dir.1) as usize;
                    let ny = ((y as i32) + dir.0) as usize;

                    if let CellType::Gear{id} = grid[ny][nx] {
                        gears.push(CellType::Gear{id});
                    }
                }
            
                while let CellType::Num{part:_, num} = grid[y][x] {
                    num_sum *= 10;
                    num_sum += num;
                    
                    // Check for gears above and below
                    for dir in [ADJACENT_DIRS[3], ADJACENT_DIRS[7]] {
                        let nx = ((x as i32) + dir.1) as usize;
                        let ny = ((y as i32) + dir.0) as usize;
                        
                        if let CellType::Gear{id} = grid[ny][nx] {
                            gears.push(CellType::Gear{id});
                        }
                    }

                    x += 1;
                }

                x-=1;

                // Check for gears to the right
                for dir in ADJACENT_DIRS[0..3].iter() {
                    let nx = ((x as i32) + dir.1) as usize;
                    let ny = ((y as i32) + dir.0) as usize;

                    if let CellType::Gear{id} = grid[ny][nx] {
                        gears.push(CellType::Gear{id});
                    }
                }

                // At this point num_sum is the whole number
                // and gears is a list of gears adjacent to the number

                for attached_gear in gears {
                    if let CellType::Gear{id} = attached_gear {
                        gear_ratios[id as usize].0 *= num_sum;
                        gear_ratios[id as usize].1 += 1;
                    }
                }
            }
            x+=1;
        }
        x = 1;
        y+=1;
    }

    let mut sum = 0;

    for (gear_ratio, _) in gear_ratios.iter().filter(|(_, count)| *count == 2) {
        sum += gear_ratio;
    }
    
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1() {
        let input = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn day3_part2() {
        let input =
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.." ;
        assert_eq!(part2(input), 467835);
    }
}
