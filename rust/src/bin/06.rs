pub fn part_one(input: &str) -> Option<u32> {
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        // let current: &char = all_chars.get(i).unwrap();
        let mut next_4 = vec![chars[i], chars[i+1], chars[i+2], chars[i+3]];
        next_4.sort();
        next_4.dedup();
        if next_4.len() != 4 {
            continue;
        }
        return Some((i + 4) as u32);
    }
    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {

    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        let mut next_14: Vec<char> = Vec::new();
        for j in i..(i+14) {
            next_14.push(*chars.get(j).unwrap())
        }

        next_14.sort();
        next_14.dedup();
        if next_14.len() != 14 {
            continue;
        }
        return Some((i + 14) as u32);
    }
    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
