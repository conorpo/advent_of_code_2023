use colored::Colorize;

fn main() {
    let input = include_str!("./10.txt");
    let part1_output = part1(input);
    dbg!(part1_output);
    let part2_output = part2(input);
    dbg!(part2_output);
}

fn part1(input: &str) -> u32 {
    // 4 least significant bits represent connections to the 4 cardinal directions, NESW
    let mut bitset_grid = Vec::new();
    let mut starting_point = None;
    
    let w = input.lines().next().unwrap().chars().count();

    bitset_grid.push(vec![0b0000; w]);
    for (i, line) in input.lines().enumerate() {
        bitset_grid.push(Vec::new());
        bitset_grid[i+1].push(0b0000);
        for (j, c) in line.chars().enumerate() {
            let cell_bitset: u8 = match c {
                '.' => 0b0000,
                '|' => 0b1010,
                '-' => 0b0101,
                'L' => 0b1100,
                'F' => 0b0110,
                '7' => 0b0011,
                'J' => 0b1001,
                'S' => {
                    starting_point = Some((j + 1, i + 1));
                        0b1111
                    },
                _ => panic!("Invalid character in input"),
            };
            bitset_grid[i+1].push(cell_bitset);
        }
        bitset_grid[i+1].push(0b0000);
    }
    bitset_grid.push(vec![0b0000; w]);

    //dbg!(starting_point);

    // [N, E, S, W],  (X, Y)
    const DIRS:[(isize, isize);4]= [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut cur_positons = Vec::new();
    let mut last_dir = Vec::new();

    if let Some(s_pos) = starting_point {
        DIRS.to_vec().iter().enumerate().for_each(|(i,(dx,dy))| {
            let x = s_pos.0;
            let y = s_pos.1;
            
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            
            // (i+2)%4 gets the opposite direction, 8 >> (i+2)%4 gets the bit for the opposite direction
            let opposite_dir_bitset: u8 = 8u8 >> (i+2)%4;
            if bitset_grid[ny][nx] & opposite_dir_bitset != 0 {
                cur_positons.push((nx, ny));
                last_dir.push(opposite_dir_bitset);
            };
            
            //dbg!((nx, ny), format!(" {:04b}",opposite_dir_bitset) , format!("{:04b}",bitset_grid[ny][nx]));
            //println!("");
        });
    }
    
    assert_eq!(cur_positons.len(), 2); // 2 possible directions to go from the starting point
    assert_eq!(last_dir.len(), 2);

    let mut flip_flop = 0;   
    let mut dist = 1; // Our 2 positons are already 1 step away from the starting point
    while cur_positons[0] != cur_positons[1] {
        let (x, y) = cur_positons[flip_flop];

        // dbg!(flip_flop);
        // println!("(x:{:}, y:{:})", x, y);
        
        let next_dir_bitset = bitset_grid[y][x] ^ last_dir[flip_flop];
        //println!("{:04b}\n{:04b}\n{:04b}\n", bitset_grid[y][x], last_dir[flip_flop], next_dir_bitset);

        // Trailing zeroes will convert 0b0001 to 0, 0b1000 to 3. 3 - that will give the correct index
        let next_dir_index = 3 - next_dir_bitset.trailing_zeros() as usize;
        
        let (dx, dy) = DIRS[next_dir_index];
        
        let nx = (x as isize + dx) as usize;     
        let ny = (y as isize + dy) as usize;
        
        cur_positons[flip_flop] = (nx, ny);       
        last_dir[flip_flop] = 8u8 >> ((2 + next_dir_index)%4); // (2 + next_dir_index)%4 gets the opposite direction index, 8 >> that gets the bitset for the opposite direction

        dist += flip_flop; //Only increase dist after both positions have been updated, as dist is min distance from S in either direction
        flip_flop = (flip_flop + 1) % 2;
    }
    
    //println!("{:?}\n\n", dist);

    dist as u32
}

fn part2(input: &str) -> u32 {
    // 4 least significant bits represent connections to the 4 cardinal directions, NESW
    let mut bitset_grid = Vec::new();
    let mut s_pos = (0, 0);
    
    let w = input.lines().next().unwrap().chars().count();

    bitset_grid.push(vec![0b0000; w]);
    for (i, line) in input.lines().enumerate() {
        bitset_grid.push(Vec::new());
        bitset_grid[i+1].push(0b0000);
        for (j, c) in line.chars().enumerate() {
            let cell_bitset: u8 = match c {
                '.' => 0b0000,
                '|' => 0b1010,
                '-' => 0b0101,
                'L' => 0b1100,
                'F' => 0b0110,
                '7' => 0b0011,
                'J' => 0b1001,
                'S' => {
                        s_pos = (j + 1, i + 1);
                        0b10000
                    },
                _ => panic!("Invalid character in input"),
            };
            bitset_grid[i+1].push(cell_bitset);
        }
        bitset_grid[i+1].push(0b0000);
    }
    bitset_grid.push(vec![0b0000; w]);

    //dbg!(starting_point);

    // [N, E, S, W],  (X, Y)
    const DIRS:[(isize, isize);4]= [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut cur_pos = s_pos;
    let mut last_dir = 0;

    DIRS.to_vec().iter().enumerate().for_each(|(i,(dx,dy))| {            
        let nx = (s_pos.0 as isize + dx) as usize;
        let ny = (s_pos.1 as isize + dy) as usize;
        
        // (i+2)%4 gets the opposite direction, 8 >> (i+2)%4 gets the bit for the opposite direction
        let opposite_dir_bitset: u8 = 8u8 >> (i+2)%4;
        if bitset_grid[ny][nx] & opposite_dir_bitset != 0 {
            cur_pos = (nx, ny);
            last_dir = opposite_dir_bitset;
            bitset_grid[s_pos.1][s_pos.0] |= 8u8 >> i;
        };
        //dbg!((nx, ny), format!(" {:04b}",opposite_dir_bitset) , format!("{:04b}",bitset_grid[ny][nx]));
        //println!("");
    });

    while cur_pos != s_pos {
        let (x, y) = cur_pos;
        bitset_grid[y][x] |= 0b10000; // Mark the section of the pipe loop
        // dbg!(flip_flop);
        // println!("(x:{:}, y:{:})", x, y);
        
        let next_dir_bitset = bitset_grid[y][x] ^ last_dir;

        // Trailing zeroes will convert 0b0001 to 0, 0b1000 to 3. 3 - that will give the correct index
        let next_dir_index = 3 - next_dir_bitset.trailing_zeros() as usize;
        
        let (dx, dy) = DIRS[next_dir_index];
        
        let nx = (x as isize + dx) as usize;     
        let ny = (y as isize + dy) as usize;
        
        cur_pos = (nx, ny);       
        last_dir = 8u8 >> ((2 + next_dir_index)%4); // (2 + next_dir_index)%4 gets the opposite direction index, 8 >> that gets the bitset for the opposite direction
    }

    // Part 2, traverse the grid from left to right for each row, keeping track of if we are in the loop or not
    let mut in_loop_count = 0;
    for (y, line) in input.lines().enumerate() {
        let mut in_loop = false;
        let mut enter_direction = 0;

        for (x, c) in line.chars().enumerate() {
            let cell = bitset_grid[y+1][x+1];
            if cell & 0b10000 == 0 {
                // Empty square or random pipe
                in_loop_count += in_loop as u32;
                print!(" {} ", c.to_string().color({
                    if in_loop {
                        "blue"
                    } else {
                        "red"
                    }
                }));
            } else {
                if cell &  0b1010 != 0 {
                    if cell & 0b0100 != 0 { // Pipe continues to the right
                        enter_direction = cell & 0b1010;
                    } else {
                        in_loop = in_loop ^ ((cell & enter_direction) == 0);
                        enter_direction = 0;
                        // XOR means if right expression is true, we toggle
                        // vertical bar means cell & enter_direction == 0
                        // if we just finished a pipe segment, then
                        // cell & enter_direction == 0  - when the directions dont match
                        // cell & enter_direction != 0  - when the directions match
                    }
                }
                print!(" {} ", c.to_string().color("white"));
            }
        }
        println!("");
    }

    in_loop_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = 
".....
.S-7.
.|.|.
.L-J.
.....";

    const TEST_INPUT2: &str =
"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4);
        assert_eq!(part1(TEST_INPUT2), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
