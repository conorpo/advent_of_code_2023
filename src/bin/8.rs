use core::panic;
use std::collections::HashMap;

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}


fn main() {
    let input = include_str!("./8.txt");
    let part1_output = part1(input);
    dbg!(part1_output);
    let part2_output = part2(input);
    dbg!(part2_output);
}

struct LoopedChars<'a> {
    data: &'a str,
    iter: std::str::Chars<'a>,
}

impl<'a> LoopedChars<'a> {
    fn new(data: &'a str) -> Self {
        Self {
            data,
            iter: data.chars(),
        }
    }

    fn next(&mut self) -> Option<char> {
        if let Some(next) = self.iter.next() {
            Some(next)
        } else {
            self.iter = self.data.chars();
            self.iter.next()
        }
    }
}


fn part1(input: &str) -> u32 {
    let (instructions, graph) = input.split_once("\n\n").unwrap();

    // Yes I realize that this is O(mlog(n)) because it does the hashing every step, but I want to learn more about Rust heap allocation, smart pointers, etc.. before I attempt that.

    let mut node_map:HashMap<&str, Node> = HashMap::new();
    let graph = graph.lines().map(|l| l.split_once(" = ").unwrap());
    let mut zzz_ref: *const _ = std::ptr::null();
    
    for (node, children) in graph {
        let (mut left, mut right) = children.split_once(", ").unwrap();
        left = &left[1..];
        right = &right[..3];

        //dbg!(left, right);

        node_map.insert(node, Node {left,right});

        if node == "ZZZ" {
            zzz_ref = node_map.get(node).unwrap() as *const _;
        }
    }

    let mut steps = 0;

    let mut cur_node = node_map.get("AAA").unwrap();
    let mut looped_instructions = LoopedChars::new(instructions);

    while cur_node as *const _ != zzz_ref {
        //dbg!(cur_node);
        let dir = looped_instructions.next().unwrap();
        match dir {
            'L' => cur_node = node_map.get(cur_node.left).unwrap(),
            'R' => cur_node = node_map.get(cur_node.right).unwrap(),
            _ => panic!("Invalid direction")
        }
        steps += 1;
    }    

    steps
}



fn part2(input: &str) -> u64 {
    // Okay time to do a smarter solution, input data is many disconnected graphs.
    // Also after doing some testing, the instructions are made in a way that there is only 1 cycle, and it always is on the last instruction when it reaches the end node.

    let (instructions, graph) = input.split_once("\n\n").unwrap();

    let mut node_map:HashMap<&str, usize> = HashMap::new();
    let mut starting_nodes = Vec::new();

    
    for (i,line) in graph.lines().enumerate() {
        let (node, _) = line.split_once(" = ").unwrap();
        node_map.insert(node, i);

        if node.ends_with('A') {
            starting_nodes.push(i);
        }
    }

    let mut nodes = Vec::new();
    
    for (node , children) in graph.lines().map(|l| l.split_once(" = ").unwrap()) {
        let (left, right) = children.split_once(", ").unwrap();

        let left = node_map.get(&left[1..]).unwrap();
        let right = node_map.get(&right[..3]).unwrap();
        let is_end = node.ends_with('Z');

        nodes.push((*left, *right, is_end));
    }

    let mut looped_instructions = LoopedChars::new(instructions);

    let mut steps = 0;
    let loop_size = instructions.chars().count();


    let mut loop_patterns = vec![(0,0,0,Vec::new()); starting_nodes.len()];
    let mut loop_pattern_count = 0;

    while let Some(dir) = looped_instructions.next() {        
        steps += 1;
        
        for (j,idx) in starting_nodes.iter_mut().enumerate() {
            *idx = match dir {
                'L' => nodes[*idx].0,
                'R' => nodes[*idx].1,
                _ => panic!("Invalid direction")
            };

            if nodes[*idx].2  {
                if loop_patterns[j].0 == 0 {
                    loop_patterns[j] = (steps, steps % loop_size, 0, Vec::new());    
                } else if steps % loop_size == loop_patterns[j].1 && loop_patterns[j].2 == 0 {
                    loop_patterns[j].2 = steps - loop_patterns[j].0; // Loop size
                    loop_pattern_count += 1;
                } else if loop_patterns[j].2 == 0 { // Not on the same instruction, but valid end
                    let last = *loop_patterns[j].3.last().unwrap_or(&loop_patterns[j].0);
                    loop_patterns[j].3.push(steps - last);
                }
            }
        }
        
        if loop_pattern_count == starting_nodes.len() {
            break;
        }
    }

    dbg!(&loop_patterns);
    // Turns out they literally all instantly loop (first end node at (n), loop size(n)), no touching endpoint inbetween 
    // Despite the fact that the graphs could have been much more complicated, e.g. out of phase with instructions, hitting end points multiple times per loop, etc..
    // Kind of a dumb problem.. 

    let loop_patterns = loop_patterns.iter().map(|(_,_,l,_)| (l / loop_size) as u64).collect::<Vec<u64>>();

    dbg!(&loop_patterns);
    // All prime numbers, clever...

    loop_patterns.iter().fold(1, |acc, x| acc * x) * loop_size as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const test_input: &str = 
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input), 6);
    }
}
