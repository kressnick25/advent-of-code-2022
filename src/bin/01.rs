#[derive (Clone)]
struct Elf {
    food: Vec<u32>
}

impl Elf {
    fn new() -> Elf {
        Elf {
            food: Vec::new()
        }
    }

    fn total(&self) -> u32 {
        self.food.iter().sum()
    }
}

fn parse_elves(input: &str) -> Vec<Elf> {
    let split = input.split('\n');
    let mut elves: Vec<Elf> = Vec::new();
    
    let mut new_elf = Elf::new();
    for line in split {
        if line.is_empty() {
            elves.push(new_elf);
            new_elf = Elf::new();
        } else {
            new_elf.food.push(line.parse::<u32>().unwrap());
        }
    }

    elves
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut elves = parse_elves(input);

    let mut top_elf: &Elf = &Elf { food: vec![0] };
    for elf in elves.iter_mut() {
        if elf.total() > top_elf.total() {
            top_elf = elf;
        }
    }

    Some(top_elf.total())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = parse_elves(input);
    
    let empty_elf = Elf {
        food: vec![0]
    };
    let mut elf_1 = &empty_elf;
    let mut elf_2 = &empty_elf;
    let mut elf_3 = &empty_elf;
    for elf in elves.iter_mut() {
        // no idea how to mutate multiple array elements at once
        // do this instead
        if elf.total() > elf_1.total() {
            elf_3 = elf_2;
            elf_2 = elf_1;
            elf_1 = elf;
        } else if elf.total() > elf_2.total() {
            elf_3 = elf_2;
            elf_2 = elf;
        } else if elf.total() > elf_3.total() {
            elf_3 = elf;
        }
    }


    let total = elf_1.total() + elf_2.total() + elf_3.total(); 
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
