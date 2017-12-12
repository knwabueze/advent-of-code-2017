const INPUT: &'static str = include_str!("../inputs/02.txt");

fn main() {
    println!("{}", checksum_1(INPUT));
    println!("{}", checksum_2(INPUT));
}

fn checksum_1(seq: &str) -> u32 {
    let mut acc = 0u32;

    for line in seq.lines() {
        let line = line.split_whitespace();

        let mut row_elements: Vec<_> = line.map(
            |i| i.parse::<u32>().expect("Expected an integer."),
        ).collect();

        row_elements.sort();

        acc += row_elements.last().unwrap() - row_elements.first().unwrap();
    }

    acc
}

fn checksum_2(seq: &str) -> u32 {
    let mut acc = 0u32;

    for line in seq.lines() {
        let line = line.split_whitespace();

        let mut row_elements: Vec<_> = line.map(
            |i| i.parse::<u32>().expect("Expected an integer."),
        ).collect();

        row_elements.sort();

        'outer: for (i, v1) in row_elements.iter().enumerate() {
            for v2 in row_elements.iter().skip(i + 1) {
                if v2 % v1 == 0 {
                    acc += v2 / v1;
                    break 'outer
                }
            }
        }
    }

    acc
}


#[cfg(test)]
mod star_1 {
    use super::checksum_1;

    #[test]
    fn spreadsheet_1() {
        let spreadsheet = "5 1 9 5
                            7 5 3
                            2 4 6 8";

        assert_eq!(checksum_1(spreadsheet), 18);
    }
}

#[cfg(test)]
mod star_2 {
    use super::checksum_2;

    #[test]
    fn spreadsheet_1() {
        let spreadsheet = "5 9 2 8
                            9 4 7 3
                            3 8 6 5";

        assert_eq!(checksum_2(spreadsheet), 9);
    }
}
