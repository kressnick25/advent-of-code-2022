struct Assignment {
    start: u32,
    end: u32
}

impl Assignment {
    fn encompasses(&self, other: &Assignment) -> bool {
        (self.start <= other.start && self.end >= other.end)
        || (other.start <= self.start && other.end >= self.end)
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        (self.start <= other.start && self.end >= other.end)
            || (other.start <= self.start && other.end >= self.end)
            || (self.end >= other.end && self.start <= other.end)
            || (other.end >= self.start && other.start <= self.end)
    }
}

fn parse_elf_pairs(input: &str) -> Vec<(Assignment, Assignment)> {
    input.lines()
            .into_iter()
            .map(|line| {
                let mut assignments = line.split(',');
                let mut a1 = assignments.next().unwrap().split('-');
                let mut a2 = assignments.next().unwrap().split('-');
                
                (
                    Assignment { 
                        start: u32::from_str_radix(a1.next().unwrap(), 16).unwrap(),
                        end: u32::from_str_radix(a1.next().unwrap(), 16).unwrap() 
                    },
                    Assignment { 
                        start: u32::from_str_radix(a2.next().unwrap(), 16).unwrap(),
                        end: u32::from_str_radix(a2.next().unwrap(), 16).unwrap() 
                    },
                )
            })
            .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let elf_pairs: Vec<(Assignment, Assignment)> = parse_elf_pairs(input);

    let result: u32 = elf_pairs
        .iter()
        .fold(0, |acc, (elf_a, elf_b)| {
            if elf_a.encompasses(elf_b) { acc + 1 } else { acc }
        });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elf_pairs: Vec<(Assignment, Assignment)> = parse_elf_pairs(input);

    let result: u32 = elf_pairs
        .iter()
        .map(| (elf_a, elf_b) | elf_a.overlaps(elf_b))
        .fold(0, | acc, v | if v { acc + 1 } else { acc });

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
