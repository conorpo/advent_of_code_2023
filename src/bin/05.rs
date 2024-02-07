use std::collections::VecDeque;

fn main() {
    let input = include_str!("./5.txt");
    let part1_output = part1(input);
    dbg!(part1_output);
    let part2_output = part2(input);
    dbg!(part2_output);
}

fn range_map(map: Vec<(u64,u64,u64)>, sources: Vec<u64>) -> Vec<u64> {
    let mut dests = Vec::with_capacity(sources.len());
    for source in sources {
        // Valid Sources as a filtered iter
        let mut valid_sources = map.iter().filter(|(_,s,r)| *s <= source && source < *s + *r);

        // Should only be one valid source
        match valid_sources.next() {
            Some((d,s,_)) => {
                dests.push(d+source-s);
            },
            None => {
                dests.push(source);
            }
        }
    }
    dests
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut cur_ids = lines.next().unwrap().split_once(": ").unwrap().1.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut current_map = Vec::<(u64,u64,u64)>::new();
    for line in lines {
        if line.ends_with(":") {
            current_map = Vec::new();
        } else if line.trim().is_empty() {
            // Map is made
            cur_ids = range_map(current_map.clone(), cur_ids);
        } else {
            let m: Vec<u64> = line.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
            current_map.push((m[0], m[1], m[2]));
        }
    }

    // Last map
    cur_ids = range_map(current_map, cur_ids);

    *cur_ids.iter().min().unwrap()
}


fn range_map2(map: &Vec<(u64,u64,u64)>, sources: Vec<(u64,u64)>) -> Vec<(u64,u64)> {
    let mut dests = Vec::new();

    let mut source_queue = VecDeque::from(sources);

    //dbg!(map);

    while let Some((start, range)) = source_queue.pop_front(){
        //dbg!(start, range);

        // Valid Sources as a filtered iter

        // Should only be one valid source
        let mut was_mapped = false;
        for (d,s,r) in map.iter() {
            let start_map = start.max(*s);
            let end_map = (start + range).min(*s + *r);
            
            /*
                 0      1       2       3       4       5       6       7       8       9
                      start                                start+range
                                        s              s+r
                                     start_map        end_map
                     start           start_map        end_map  start+range    
            */

            if start_map < end_map {
                dests.push((d+start_map-s, end_map-start_map));
                
                if start_map > start {
                    source_queue.push_back((start, start_map - start));
                } 

                if end_map < start + range {
                    source_queue.push_back((end_map, start + range - end_map));
                }
                
                was_mapped = true;
                break;
            }
        }

        if !was_mapped {
            dests.push((start, range));
        }
    }

    dests
}


fn part2(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut seeds_input = lines.next().unwrap().split_once(": ").unwrap().1.split_whitespace();
    let mut cur_ranges: Vec<(u64,u64)> = Vec::new();

    //dbg!(seeds_input.next().unwrap());

    while let Some(start) = seeds_input.next().and_then(|s| s.parse::<u64>().ok()) {
        //print!("{}", start);
        if let Some(range) = seeds_input.next().and_then(|s| s.parse::<u64>().ok()) {
            //println!(" {}", range);
            cur_ranges.push((start, range));
        }
    }

    let mut current_map = Vec::<(u64,u64,u64)>::new();
    
    lines.next(); // Skip the first blank line

    while let Some(line) = lines.next() {
        if line.ends_with(":") {
            current_map = Vec::new();
        } else if line.trim().is_empty() {
            // Map is made
            cur_ranges = range_map2(&current_map, cur_ranges);
            //dbg!(&cur_ranges);
        } else {
            let m: Vec<u64> = line.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
            current_map.push((m[0], m[1], m[2]));
        }
    }

    // Last map
    cur_ranges = range_map2(&current_map, cur_ranges);

    *cur_ranges.iter().map(|(s,_)| s).min().unwrap()
}



#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 46);
    }
}
