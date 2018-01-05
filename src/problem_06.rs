use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashSet;
use std::collections::HashMap;

const INPUT: &'static str = include_str!("../inputs/06.txt");

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
struct MemoryBanks {
    banks: Vec<u32>,
    len: usize,
}

impl MemoryBanks {
    fn largest_bank(&self) -> (usize, u32) {
        let mut largest: u32 = 0;
        let mut largest_i: usize = 0;

        for (i, b) in self.banks.iter().enumerate() {
            if *b > largest {
                largest = *b;
                largest_i = i;
            }
        }

        (largest_i, largest)
    }
}

impl FromStr for MemoryBanks {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut memory_banks: Vec<u32> = Vec::new();

        for initial_value in s.split_whitespace()
            .map(|l| l.parse::<u32>().expect("Error parsing string."))
        {
            memory_banks.push(initial_value);
        }

        let len = memory_banks.len();
        Ok(MemoryBanks {
            banks: memory_banks,
            len: len,
        })
    }
}

fn main() {
    println!("{}", redist_cycles(INPUT));
    println!("{}", redist_cycles_2(INPUT));
}

fn redist_cycles_2(input: &str) -> u32 {
    let mut banks = MemoryBanks::from_str(input).expect("Error in parsing strings.");
    let mut cycles = 0u32;
    let mut seen_combis: HashMap<MemoryBanks, u32> = HashMap::new();

    loop {
        seen_combis.insert(banks.clone(), cycles);

        let (i, mut value) = banks.largest_bank();
        banks.banks[i] = 0;

        let mut array_pointer = (i + 1) % banks.len;

        while value > 0 {
            banks.banks[array_pointer] = banks.banks[array_pointer] + 1;
            array_pointer = (array_pointer + 1) % banks.len;
            value -= 1;
        }

        cycles += 1;

        if seen_combis.contains_key(&banks) {
            return cycles - seen_combis[&banks];
        }
    }
}

fn redist_cycles(input: &str) -> u32 {
    let mut banks = MemoryBanks::from_str(input).expect("Error in parsing strings.");
    let mut cycles = 0u32;
    let mut seen_combis: HashSet<MemoryBanks> = HashSet::new();

    loop {
        seen_combis.insert(banks.clone());

        let (i, mut value) = banks.largest_bank();
        banks.banks[i] = 0;

        let mut array_pointer = (i + 1) % banks.len;

        while value > 0 {
            banks.banks[array_pointer] = banks.banks[array_pointer] + 1;
            array_pointer = (array_pointer + 1) % banks.len;
            value -= 1;
        }

        cycles += 1;

        if seen_combis.contains(&banks) {
            break
        }
    }

    cycles
}


#[cfg(test)]
mod tests {
    use super::redist_cycles;
    use super::redist_cycles_2;

    #[test]
    fn redist_procedure_a() {
        assert_eq!(redist_cycles("0 2 7 0"), 5);
    }

    #[test]
    fn redist_procedure_b() {
        assert_eq!(redist_cycles_2("0 2 7 0"), 4);
    }
}
