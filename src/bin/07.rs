use std::collections::HashMap;

fn main() {
    let input = include_str!("./7.txt");
    let part1_output = part1(input);
    dbg!(part1_output);
    let part2_output = part2(input);
    dbg!(part2_output);
}

fn part1(input: &str) -> u64 {
    let mut map = HashMap::new();

    map.insert('2', 'a');
    map.insert('3', 'b');
    map.insert('4', 'c');
    map.insert('5', 'd');
    map.insert('6', 'e');
    map.insert('7', 'f');
    map.insert('8', 'g');
    map.insert('9', 'h');
    map.insert('T', 'i');
    map.insert('J', 'j'); //nice
    map.insert('Q', 'k');
    map.insert('K', 'l');
    map.insert('A', 'm');

    // Mapped Hand, Bid, Type
    let mut hands: Vec<(String, u64, u32)> = Vec::new();

    for line in input.lines() {
        let (hand, bid) = line.split_once(" ").unwrap();
        let bid = bid.parse::<u64>().unwrap();

        // Hand where each card is mapped to a leter, card rank is alphabetical
        let mut mapped_hand = String::new();

        // Probably overcomplicated
        let mut card_count:HashMap<char,u32> = HashMap::new();
        for c in hand.chars() {
            mapped_hand.push(*map.get(&c).unwrap());
            *card_count.entry(c).or_insert(0) += 1;
        }

        let mut sets = card_count.values().map(|v| *v).collect::<Vec<u32>>();
        sets.sort_by(|a,b| b.cmp(a)); // Descending

        // Edge case, 5 of a kind only has 1 set
        sets.push(0);

        let set_tuple = (sets[0], sets[1]);
        let hand_type = match set_tuple {
            (5,0) => 1,
            (4,1) => 2,
            (3,2) => 3,
            (3,1) => 4,
            (2,2) => 5,
            (2,1) => 6,
            (1,1) => 7,
            _ => panic!("Invalid hand")
        };


        hands.push((mapped_hand, bid, hand_type));
    }

    hands.sort_by(|a,b| {
        if a.2 == b.2 {
            a.0.cmp(&b.0)
        } else {
            b.2.cmp(&a.2)
        }
    });


    let mut sum = 0u64;

    for (i,hand) in hands.into_iter().enumerate() {
        sum += hand.1 * (i + 1) as u64;
    }

    sum
}

fn part2(input: &str) -> u64 {
    let mut map = HashMap::new();

    map.insert('2', 'a');
    map.insert('3', 'b');
    map.insert('4', 'c');
    map.insert('5', 'd');
    map.insert('6', 'e');
    map.insert('7', 'f');
    map.insert('8', 'g');
    map.insert('9', 'h');
    map.insert('T', 'i');
    map.insert('J', ' '); // now J is a joker, lowest value card
    map.insert('Q', 'k');
    map.insert('K', 'l');
    map.insert('A', 'm');

    // Mapped Hand, Bid, Type
    let mut hands: Vec<(String, u64, u32)> = Vec::new();

    for line in input.lines() {
        let (hand, bid) = line.split_once(" ").unwrap();
        let bid = bid.parse::<u64>().unwrap();

        // Hand where each card is mapped to a leter, card rank is alphabetical
        let mut mapped_hand = String::new();

        // Probably overcomplicated
        let mut card_count:HashMap<char,u32> = HashMap::new();
        for c in hand.chars() {
            mapped_hand.push(*map.get(&c).unwrap());
            *card_count.entry(c).or_insert(0) += 1;
        }

        let joker_count = *card_count.get(&'J').unwrap_or(&0);
        card_count.insert('J', 0);

        let mut sets = card_count.values().map(|v| *v).collect::<Vec<u32>>();
        sets.sort_by(|a,b| b.cmp(a)); // Descending

        // Edge case, 5 of a kind only has 1 set
        sets.push(0);

        let set_tuple = (sets[0] + joker_count, sets[1]); // Add jokers to the highest set
        let hand_type = match set_tuple {
            (5,0) => 1,
            (4,1) => 2,
            (3,2) => 3,
            (3,1) => 4,
            (2,2) => 5,
            (2,1) => 6,
            (1,1) => 7,
            _ => panic!("Invalid hand")
        };


        hands.push((mapped_hand, bid, hand_type));
    }

    hands.sort_by(|a,b| {
        if a.2 == b.2 {
            a.0.cmp(&b.0)
        } else {
            b.2.cmp(&a.2)
        }
    });


    let mut sum = 0u64;

    for (i,hand) in hands.into_iter().enumerate() {
        sum += hand.1 * (i + 1) as u64;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const test_input: &str = 
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input), 5905);
    }
}
