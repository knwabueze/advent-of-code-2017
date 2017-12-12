const INPUT: &'static str = include_str!("../inputs/01.txt");

fn inverse_captcha(sequence: &str) -> u32 {
    let mut acc: u32 = 0;
    let len = sequence.len();

    for (a, b) in sequence.chars().zip(
        sequence
            .chars()
            .skip(len - 1)
            .chain(sequence.chars().skip(1)),
    ) {
        if a == b {
            match a.to_digit(10) {
                Some(n) => acc += n,
                None => acc += 0,
            };
        }
    }

    acc
}

fn main() {
    println!("{}", inverse_captcha(INPUT));
}

#[cfg(test)]
mod test_suite {
    use super::inverse_captcha;

    #[test]
    fn sequence_results_in_three() {
        assert_eq!(inverse_captcha("1122"), 3);
    }

    #[test]
    fn sequence_results_in_four() {
        assert_eq!(inverse_captcha("1111"), 4);
    }

    #[test]
    fn sequence_results_in_zero() {
        assert_eq!(inverse_captcha("1234"), 0);
    }

    #[test]
    fn sequence_results_in_nine() {
        assert_eq!(inverse_captcha("91212129"), 9);
    }
}
