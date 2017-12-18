use std::collections::HashSet;

const INPUT: &'static str = include_str!("../inputs/04.txt");

fn main() {
    println!("{}", valid_passphrases(INPUT));
}

fn valid_passphrases(input: &str) -> u32 {
    let mut acc = 0u32;

    'passphrase: for passphrase in input.lines() {
        let mut passwords: HashSet<&str> = HashSet::new();

        for password in passphrase.split_whitespace() {
            if passwords.contains(password) {
                continue 'passphrase
            }

            passwords.insert(password);
        }

        acc += 1;
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::valid_passphrases;

    #[test]
    fn valid_passphrases_1() {
        assert_eq!(valid_passphrases("aa bb cc dd ee"), 1);
    }

    #[test]
    fn valid_passphrases_2() {
        assert_eq!(valid_passphrases("aa bb cc dd aa"), 0);
    }

    #[test]
    fn valid_passphrases_3() {
        assert_eq!(valid_passphrases("aa bb cc dd aaa"), 1);
    }
}
