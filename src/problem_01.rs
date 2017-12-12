const INPUT: &'static str = include_str!("../inputs/01.txt");

fn inverse_captcha(sequence: &str) -> u32 {
    let mut acc: u32 = 0;

    for (a, b) in sequence
        .chars()
        .zip(sequence.chars().skip(1).chain(sequence.chars().take(1)))
    {
        if a == b {
            match a.to_digit(10) {
                Some(n) => acc += n,
                None => acc += 0,
            };
        }
    }

    acc
}

fn inverse_captcha_2(sequence: &str) -> u32 {
    let mut acc: u32 = 0;
    let len = sequence.len();

    for (a, b) in sequence
        .chars()
        .enumerate()
        .filter(|a| a.0 < len / 2)
        .zip(sequence.chars().enumerate().filter(|a| a.0 >= len / 2))
    {
        if a.1 == b.1 {
            acc += match b.1.to_digit(10) {
                Some(n) => n * 2,
                None => 0,
            };
        }
    }

    acc
}


fn main() {
    println!("{}", inverse_captcha(INPUT));
    println!("{}", inverse_captcha_2(INPUT));
}

#[cfg(test)]
mod test_suite_1 {
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

#[cfg(test)]
mod test_suite_2 {
    use super::inverse_captcha_2;

    #[test]
    fn sequence_results_in_6() {
        assert_eq!(inverse_captcha_2("1212"), 6);
    }

    #[test]
    fn sequence_results_in_0() {
        assert_eq!(inverse_captcha_2("1221"), 0);
    }

    #[test]
    fn sequence_results_in_4() {
        assert_eq!(inverse_captcha_2("123425"), 4);
    }

    #[test]
    fn sequence_results_in_12() {
        assert_eq!(inverse_captcha_2("123123"), 12);
    }

    #[test]
    fn sequence_results_in_4_2() {
        assert_eq!(inverse_captcha_2("12131415"), 4);
    }
}
