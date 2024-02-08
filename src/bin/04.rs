/*
Day 4
*/


fn main() {
    let input = include_str!("./04.txt");
    let output = part1(input);
    dbg!(output);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let (_card, game) = line.split_once(':').unwrap();
        let (winning, mine) = game.split_once('|').unwrap();

        let winning = winning.split_whitespace().filter_map(|n| n.parse::<u32>().ok()).collect::<Vec<u32>>();
        let mine = mine.split_whitespace().filter_map(|n| n.parse::<u32>().ok()).collect::<Vec<u32>>();

        let mut winning_nums = 0;

        for num in mine {
            if winning.contains(&num) {
                winning_nums += 1;
            }
        }

        sum += match winning_nums {
            0 => 0,
            n => 1 << (n - 1)
        };
    }

    sum
}

fn part2(input: &str) -> u32 {
    let card_count = input.lines().count();

    let mut copies = vec![1; card_count];

    for (i, line) in input.lines().enumerate() {
        let (_card, game) = line.split_once(':').unwrap();
        let (winning, mine) = game.split_once('|').unwrap();

        let winning = winning.split_whitespace().filter_map(|n| n.parse::<u32>().ok()).collect::<Vec<u32>>();
        let mine = mine.split_whitespace().filter_map(|n| n.parse::<u32>().ok()).collect::<Vec<u32>>();
        
        let mut j = i;
        for num in mine {
            if winning.contains(&num) {
                j+=1;
                copies[j] += copies[i];
            }
        }
    }
    
    copies.iter().fold(0, |acc, x| acc + x)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1() {
        let input = 
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part1(input), 13);
    }

    #[test]
    fn day3_part2() {
        let input =
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11" ;

        assert_eq!(part2(input), 30);
    }
}
