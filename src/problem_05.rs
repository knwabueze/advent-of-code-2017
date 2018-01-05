const INPUT: &'static str = include_str!("../inputs/05.txt");

fn main() {
    println!("{}", jump_sequence(INPUT));
    println!("{}", jump_sequence_2(INPUT));
}

fn jump_sequence(input: &str) -> u32 {
    let mut steps = 0u32;
    let mut current_index = 0i32;

    let mut offset_list: Vec<i32> = input
        .lines()
        .map(|l| {
            let l = l.trim();
            l.parse::<i32>().expect("invalid integer")
        })
        .collect();

    loop {
        let current_value = offset_list[current_index as usize];
        offset_list[current_index as usize] = current_value + 1;
        current_index += current_value;
        steps += 1;

        if current_index >= offset_list.len() as i32 {
            break;
        }
    }

    steps
}

fn jump_sequence_2(input: &str) -> u32 {
    let mut steps = 0u32;
    let mut current_index = 0i32;

    let mut offset_list: Vec<i32> = input
        .lines()
        .map(|l| {
            let l = l.trim();
            l.parse::<i32>().expect("invalid integer")
        })
        .collect();

    loop {
        {
            let current_value = &mut offset_list[current_index as usize];
            current_index += *current_value;
            steps += 1;

            *current_value += if *current_value >= 3 { -1 } else { 1 };
        }

        if current_index >= offset_list.len() as i32 {
            break;
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::{jump_sequence as js, jump_sequence_2 as js2};

    #[test]
    fn jump_sequence_a() {
        let input: &str = "0
        3
        0
        1
        -3";

        assert_eq!(js(input), 5);
    }

    #[test]
    fn jump_sequence_b() {
        let input: &str = "0
        3
        0
        1
        -3";

        assert_eq!(js2(input), 10);
    }
}
