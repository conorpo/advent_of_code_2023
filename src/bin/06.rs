fn main() {
    let input = include_str!("./06.txt");
    let part1_output = part1(input);
    dbg!(part1_output);
    let part2_output = part2(input);
    dbg!(part2_output);
}

fn part1(input: &str) -> u32 {
    let mut sum = 1;

    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace().skip(1).map(|s| s.parse::<u32>().unwrap());
    let distances = lines.next().unwrap().split_whitespace().skip(1).map(|s| s.parse::<u32>().unwrap());

    // Formula
    // Distance tx - x^2 = d  (where x is amount pushing the button)
    
    for (t,d) in times.zip(distances) {
        let det = ((t*t - 4*d) as f64).sqrt();

        const EPSILON: f64 = 0.001;
        let x1 = (-(t as f64) + det) / -2.0 + EPSILON; // Add epsilon to make sure we go FURTHER
        let x2 = (-(t as f64) - det) / -2.0 - EPSILON; // Subtract epsilon to make sure we go FURTHER

        //x1 will always be the smaller value
        let range = (x1.ceil() as u32).abs_diff(x2.floor() as u32);

        sum *= range + 1;
    }

    sum
}

fn part2(input: &str) -> u32 {
    let mut lines = input.lines();
    let t = lines.next().unwrap().split_once(":").unwrap().1.replace(" ", "").parse::<u64>().unwrap();
    let d = lines.next().unwrap().split_once(":").unwrap().1.replace(" ", "").parse::<u64>().unwrap();

    // Formula
    // Distance tx - x^2 = d  (where x is amount pushing the button)

    let det = ((t*t - 4*d) as f64).sqrt();

    const EPSILON: f64 = 0.001;
    let x1 = (-(t as f64) + det) / -2.0 + EPSILON; // Add epsilon to make sure we go FURTHER
    let x2 = (-(t as f64) - det) / -2.0 - EPSILON; // Subtract epsilon to make sure we go FURTHER

    //x1 will always be the smaller value
    let range = (x1.ceil() as u32).abs_diff(x2.floor() as u32);

    range + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const test_input: &str = 
"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input), 71503);
    }
}
