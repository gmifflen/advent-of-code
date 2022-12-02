use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Hand {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        if self == &Hand::Scissors && other == &Hand::Rock {
            Some(Ordering::Less)
        } else if self == &Hand::Rock
            && other == &Hand::Scissors
        {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<Hand> = line
                .split(" ")
                .map(|s| s.parse::<Hand>().unwrap())
                .collect();
            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Equal) => {
                    3 + moves[1] as u32
                }
                Some(Ordering::Less) => 6 + moves[1] as u32,
                Some(Ordering::Greater) => {
                    0 + moves[1] as u32
                }
                None => {
                    panic!("moves should be comparable")
                }
            }
        })
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<&str> =
                line.split(" ").collect();
            let opponent_move =
                moves[0].parse::<Hand>().unwrap();
            match moves[1] {
                "X" => {
                    let our_move = match opponent_move {
                        Hand::Rock => Hand::Scissors,
                        Hand::Paper => Hand::Rock,
                        Hand::Scissors => Hand::Paper,
                    };
                    0 + our_move as u32
                }
                "Y" => 3 + opponent_move as u32,
                "Z" => {
                    let our_move = match opponent_move {
                        Hand::Rock => Hand::Paper,
                        Hand::Paper => Hand::Scissors,
                        Hand::Scissors => Hand::Rock,
                    };
                    6 + our_move as u32
                }
                _ => {
                    panic!("Unexpected response");
                }
            }
        })
        .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
