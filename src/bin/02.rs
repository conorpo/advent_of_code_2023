//Puzzle Input
fn main() {
    let input = include_str!("./02.txt");
    let output = part1(input);
    dbg!(output);
    let output2 = part2(input);
    dbg!(output2);
}


fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for (game_id, game_str) in input.lines().enumerate() {
        let (_, rounds) = game_str.split_once(':').unwrap();

        let mut possible = true;

        for round in rounds.split(';') {
            for info in round.split(',') {
                if let Some((count, color)) = info.trim().split_once(' ') {
                    let count = count.parse::<u32>().unwrap();

                    possible = match color {
                        "red" => count <= 12,
                        "green" => count <= 13,
                        "blue" => count <= 14,
                        _ => true
                    };

                    if !possible {
                        break;
                    }
                }
            }

            if !possible {
                break;
            }
        }
        if possible {
            sum += game_id as u32 + 1;
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for game in input.lines() {
        let (_, rounds) = game.split_once(':').unwrap();
        let mut max_cubes = (0,0,0);
        
        for round in rounds.split(';') {
            for info in round.split(',') {
                if let Some((count, color)) = info.trim().split_once(' ') {
                    let count = count.parse::<u32>().unwrap();

                    match color {
                        "red" => max_cubes.0 = max_cubes.0.max(count),
                        "green" => max_cubes.1 = max_cubes.1.max(count),
                        "blue" => max_cubes.2 = max_cubes.2.max(count),
                        _ => {}
                    }
                }
            }
        }
        
        let power_set = max_cubes.0 * max_cubes.1 * max_cubes.2;
        sum += power_set;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test__both() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), 8);
        assert_eq!(part2(input), 2286);
    }
}
