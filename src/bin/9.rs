// Gonna start combining parts when they're similar enough

fn main() {
    let input = include_str!("./9.txt");
    let (part1_output, part2_output) = both_parts(input);

    dbg!(part1_output);
    dbg!(part2_output);
}

fn both_parts(input: &str) -> (i32, i32) {
    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for line in input.lines() {
        let mut history = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let mut first_of_each = Vec::new();
        let mut last_of_each = Vec::new();

        while {
            first_of_each.push(*history.first().unwrap());
            last_of_each.push(*history.last().unwrap());

            let mut all_zeroes = true;
            
            for i in 1..history.len() {
                let dif = history[i] - history[i-1];

                all_zeroes &= dif == 0;
                history[i-1] = dif;
            }
            history.pop();

            !all_zeroes
        } {}

        // Extrapolate
        let prev_value = first_of_each.iter().rev().fold(0, |acc, x| x - acc);
        let next_value = last_of_each.iter().sum::<i32>();
        
        part1_sum += next_value;
        part2_sum += prev_value;
    }

    (part1_sum, part2_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = 
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_both_parts() {
        assert_eq!(both_parts(TEST_INPUT), (114, 2));
    }
}
