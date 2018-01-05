use std::collections::HashSet;

const INPUT: &'static str = include_str!("../inputs/04.txt");

fn main() {
    println!("{}", valid_passphrases(INPUT));
    println!("{}", valid_passphrases_2(INPUT));
}

fn valid_passphrases(input: &str) -> u32 {
    let mut acc = 0u32;

    'passphrase: for passphrase in input.lines() {
        let mut passwords: HashSet<&str> = HashSet::new();

        for password in passphrase.split_whitespace() {
            if passwords.contains(password) {
                continue 'passphrase;
            }

            passwords.insert(password);
        }

        acc += 1;
    }

    acc
}

fn valid_passphrases_2(input: &str) -> u32 {
    let mut acc = 0u32;

    'passphrase: for passphrase in input.lines() {
        let mut passwords: HashSet<Box<[u32]>> = HashSet::new();

        for password in passphrase.split_whitespace() {
            if passwords.contains(&deanagramize(password)) {
                continue 'passphrase;
            }

            passwords.insert(deanagramize(password));
        }

        acc += 1;
    }

    acc
}

fn deanagramize(input: &str) -> Box<[u32]> {
    let ret: Vec<u32> = input.chars().map(|v| v as u32).collect();
    let mut ret: Box<[u32]> = ret.into_boxed_slice();
    ret.sort();

    ret
}

#[cfg(test)]
mod tests {
    use super::valid_passphrases;
    use super::valid_passphrases_2 as vp_2;

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

    #[test]
    fn valid_passphrases_4() {
        assert_eq!(vp_2("abcde fghij"), 1);
    }

    #[test]
    fn valid_passphrases_5() {
        assert_eq!(vp_2("abcde xyz ecdab"), 0);
    }
}
