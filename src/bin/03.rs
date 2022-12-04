use std::str::Lines;
use std::collections::HashSet;

struct Rucksack {
    first_compartment: HashSet<char>,
    second_compartment: HashSet<char> 
}

impl Rucksack {
    fn from_str(val:  &str) -> Rucksack {
        Rucksack {
            first_compartment: val[val.len() / 2..].chars().into_iter().collect::<HashSet<char>>(),
            second_compartment: val[..val.len() / 2].chars().into_iter().collect::<HashSet<char>>()
        }
    }

    fn find_similar_char(&self) ->  Option<char> {
        for letter in &self.first_compartment {
            if self.second_compartment.contains(letter) {
                return Some(*letter)
            }
        }
        None
    }

    fn contains(&self, val: &char) -> bool {
        self.first_compartment.contains(val) || self.second_compartment.contains(val)
    }

    fn full_sack(&self) -> HashSet<char> {
        let mut new_set: HashSet<char> = HashSet::new();

        for item in self.first_compartment.iter() {
            new_set.insert(*item);
        }

        for item in self.second_compartment.iter() {
            new_set.insert(*item);
        }

        new_set
    }

}

struct Group<'a> {
    elf_1: &'a Rucksack,
    elf_2: &'a Rucksack,
    elf_3: &'a Rucksack
}

impl Group<'_> {
    fn find_similar_char(&self) -> Option<char> {
        self.elf_1.full_sack()
                  .into_iter()
                  .find(|&item| self.elf_2.contains(&item) && self.elf_3
                  .contains(&item))
    }
}

fn get_worth(item: &char) -> u32{
    let upper_correction: u32 = 38;
    let lower_correction: u32 = 96;
    let int_val = *item as u32;

    if item.is_uppercase() {
        int_val - upper_correction
      } else {
        int_val - lower_correction
    }

}

fn get_elves(input: &str) -> Vec<Rucksack> {
    let packs: Lines<'_> = input.lines();

     packs.into_iter()
          .map(Rucksack::from_str)
          .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let elves = get_elves(input);

    let priority_items: Vec<char> = elves.iter()
                                         .filter_map(Rucksack::find_similar_char)
                                         .collect();
    
    Some(priority_items.iter().map(get_worth).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves = get_elves(input);

    // organize into groups
    let mut groups: Vec<Group> = Vec::new();
    for i in (0..elves.len()).step_by(3) {
        groups.push(Group {
            elf_1: elves.get(i).unwrap(),
            elf_2: elves.get(i+1).unwrap(),
            elf_3: elves.get(i+2).unwrap()
        });
    }

    let priority_items: Vec<char> = groups.iter()
                                          .filter_map(Group::find_similar_char)
                                          .collect();

    Some(priority_items.iter().map(get_worth).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
