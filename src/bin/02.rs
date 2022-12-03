// Rock     A   Y   1
// Paper    B   X   2
// Scissors C   Z   3 

// Win  6
// Draw 3
// Lose 0

#[derive(PartialEq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

impl Hand {
    fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    }

    fn from_opponent(val: &str) -> Hand {
        match val {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("Invalid hand value for char {}", val)
        }
    }

    fn from_player(val: &str) -> Hand {
        match val {
            "Y" => Hand::Paper,
            "X" => Hand::Rock,
            "Z" => Hand::Scissors,
            _ => panic!("Invalid hand value for char {}", val)
        }
    }
}

#[derive(Clone)]
struct Match {
    opponent: Hand,
    player: Option<Hand>,
    result: Option<MatchResult>
}

impl Match {
    fn result(&self) -> MatchResult {
        match &self.player.as_ref().unwrap() {
            Hand::Rock => {
                match &self.opponent {
                    Hand::Rock => MatchResult::Draw,
                    Hand::Paper => MatchResult::Lose,
                    Hand::Scissors => MatchResult::Win
                }
            },
            Hand::Paper => {
                match &self.opponent {
                    Hand::Rock => MatchResult::Win,
                    Hand::Paper => MatchResult::Draw,
                    Hand::Scissors => MatchResult::Lose
                }
            },
            Hand::Scissors => {
                match &self.opponent {
                    Hand::Rock => MatchResult::Lose,
                    Hand::Paper => MatchResult::Win,
                    Hand::Scissors => MatchResult::Draw
                }
            }
        }
    }

    fn calc_hand(&self) -> Hand {
        match &self.result.as_ref().unwrap() {
            MatchResult::Draw => self.opponent.clone(),
            MatchResult::Win => {
                match &self.opponent {
                    Hand::Rock => Hand::Paper,
                    Hand::Paper => Hand::Scissors,
                    Hand::Scissors => Hand::Rock
                }
            },
            MatchResult::Lose => {
                match &self.opponent {
                    Hand::Rock => Hand::Scissors,
                    Hand::Paper => Hand::Rock,
                    Hand::Scissors => Hand::Paper
                }
                
            }
        }
    }

    fn score(self) -> u32 {
       self.result() as u32 + self.player.unwrap().score()
    }

    fn score_2(self) -> u32 {
        self.calc_hand().score() as u32 + self.result.unwrap() as u32
    }
}

#[derive(Clone)]
enum MatchResult {
    Win = 6,
    Draw = 3,
    Lose = 0
}

impl MatchResult {
    fn from_char(val: &str) -> MatchResult {
        match val {
            "X" => MatchResult::Lose,
            "Y" => MatchResult::Draw,
            "Z" => MatchResult::Win,
            _ => panic!("Invalid char for result: {}", val)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let match_lines = input.split('\n');
    
    let mut matches: Vec<Match> = Vec::new();
    for line in match_lines {
        let mut plays = line.split(' ');
        let new_match = Match {
            opponent: Hand::from_opponent(plays.next().unwrap()),
            player: Some(Hand::from_player(plays.next().unwrap())),
            result: None
        };
        matches.push(new_match);
    }

    Some(matches.iter().map(|m| m.clone().score()).sum::<u32>())
    
}

pub fn part_two(input: &str) -> Option<u32> {
    let match_lines = input.split('\n');
    
    let mut matches: Vec<Match> = Vec::new();
    for line in match_lines {
        let mut plays = line.split(' '); 
        let new_match = Match {
            opponent: Hand::from_opponent(plays.next().unwrap()),
            player: None,
            result: Some(MatchResult::from_char(plays.next().unwrap()))
        };
        matches.push(new_match);
    }

    Some(matches.iter().map(|m| m.clone().score_2()).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
