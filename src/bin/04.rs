struct Assignment {
    start: u32,
    end: u32
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
        

    let mut result = 0;
    for (elf_a, elf_b) in elf_pairs {
        if (elf_a.start <= elf_b.start && elf_a.end >= elf_b.end)
            || (elf_b.start <= elf_a.start && elf_b.end >= elf_a.end) {
            result += 1
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elf_pairs: Vec<(Assignment, Assignment)> = parse_elf_pairs(input);

    let mut result = 0;
    for (elf_a, elf_b) in elf_pairs {
        if (elf_a.start <= elf_b.start && elf_a.end >= elf_b.end)
            || (elf_b.start <= elf_a.start && elf_b.end >= elf_a.end)
            || (elf_a.end >= elf_b.end && elf_a.start <= elf_b.end)
            || (elf_b.end >= elf_a.start && elf_b.start <= elf_a.end) {
            result += 1
        }
    }

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
