const INPUT: u32 = 347991;

fn main() {
    println!("{}", shortest_distance(INPUT));
}

fn distance(input: u32, values: &[u32]) -> u32 {
    let mut dist = u32::max_value();    

    for number in values.iter().map(|x| input as i32 - *x as i32) {
        let val = number.abs() as u32;
        if val < dist {
            dist = val;
        }
    }

    
    dist
}

fn shortest_distance(input: u32) -> u32 {
    let mut steps = 0u32;

    if input == 1 {
        return 0;
    }

    for (i, v) in (0..).map(|x| (2 * x + 1 as u32).pow(2)).enumerate() {
        if input < v {
            let i = i as u32;
            steps += i;
            steps += distance(input, &[v - i * 1, v - i * 3, v - i * 5, v - i * 7]);
            break
        } 
    }

    steps
}

#[cfg(test)]
mod star_1 {
    use super::shortest_distance;

    #[test]
    fn mem_1() {
        assert_eq!(shortest_distance(1), 0);
    }

    #[test]
    fn mem_2() {
        assert_eq!(shortest_distance(12), 3);
    }

    #[test]
    fn mem_3() {
        assert_eq!(shortest_distance(23), 2);
    }

    #[test]
    fn mem_4() {
        assert_eq!(shortest_distance(1024), 31);
    }
}
