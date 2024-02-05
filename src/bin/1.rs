//Puzzle Input
pub fn main() {
    let input = include_str!("./1.txt");
    let output = part1(input);
    dbg!(output);
    let output = part2(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        // Find first and last number
        let mut first_iter = line.chars();
        let mut last_iter = line.chars().rev();

        if let Some(first) = first_iter.by_ref().find(|c| c.is_digit(10)) {
            sum += first.to_digit(10).unwrap() * 10;
        }
        if let Some(last) = last_iter.by_ref().find(|c| c.is_digit(10)) {
            sum += last.to_digit(10).unwrap();
        }
    }

    sum
}

const DIGITS: [&str;9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];


fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        // Find first and last number
        let mut iter = line.chars();
        
        let mut first_found = false;
        let mut current_number = 0;
        let mut cur_string = String::new();
        //let mut debug_num = 0;

        while let Some(c) = iter.next() {
            match c {
                c if c.is_digit(10) => {
                    current_number = c.to_digit(10).unwrap();
                    cur_string = String::new();
                },
                c => {
                    cur_string.push(c);
                    //dbg!(&cur_string);
                    let mut matching_digits = DIGITS.iter().enumerate().filter(|(_, d)| cur_string.ends_with(*d));
                    
                    if let Some(ele) = matching_digits.next() {
                        //dbg!(ele.0, ele.1, &cur_string);
                        current_number = (ele.0 as u32) + 1;
                    }
                }
            }
            
            if !first_found && current_number != 0 {
                // Add first number
                sum += current_number * 10;
                //debug_num = current_number * 10;
                first_found = true;
            }        
        }
        // Add last number
        sum += current_number;
        //debug_num += current_number;
        
        // dbg!(debug_num);
        // println!("");
    }
    
    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
one2three4five";
        assert_eq!(part2(input), 296);
    }
}