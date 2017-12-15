const INPUT: u32 = 347991;

fn main() {
    println!("{}", shortest_distance(INPUT));
    println!("{}", shortest_distance2(INPUT));
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

fn xy(x: u32, y: u32, breadth: u32) -> usize {
    ((y * breadth) + x) as usize
}

enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn next_dir(&self) -> Self {
        match self {
            &Direction::Right => Direction::Up,
            &Direction::Up => Direction::Left,
            &Direction::Left => Direction::Down,
            &Direction::Down => Direction::Right,
        }
    }

    fn as_vertex(&self, index: (u32, u32)) -> (u32, u32) {
        match self {
            &Direction::Right => (index.0 + 1, index.1),
            &Direction::Up => (index.0, index.1 + 1),
            &Direction::Left => (index.0 - 1, index.1),
            &Direction::Down => (index.0, index.1 - 1),
        }
    }
}

fn shortest_distance2(input: u32) -> u32 {
    let mut size = 0u32;
    let mut offset = 0u32;

    for (i, v) in (0..).map(|x| (2 * x + 1 as u32).pow(2)).enumerate() {
        if input <= v {
            let v = (v as f32).sqrt() as u32; // this has to be an odd square
            let i = i as u32;
            size = v;
            offset = i;
            println!("{}", size);
            break;
        }
    }

    let mut grid: Vec<u32> = vec![0; (size * size) as usize];
    let mut full_size = size * size;

    let mut index = (size - offset, size - offset);
    let mut current_direction = Direction::Right;

    let access_grid = |index: usize, grid: &mut Vec<u32>| {
        if index as u32 >= full_size {
            return 0;
        }

        grid[index as usize]
    };

    loop {
        let up = access_grid(xy(index.0, index.1 + 1, size), &mut grid);
        let down = access_grid(xy(index.0, index.1 - 1, size), &mut grid);
        let right = access_grid(xy(index.0 + 1, index.1, size), &mut grid);
        let left = access_grid(xy(index.0 - 1, index.1, size), &mut grid);

        let up_left = access_grid(xy(index.0 - 1, index.1 + 1, size), &mut grid);
        let down_right = access_grid(xy(index.0 + 1, index.1 - 1, size), &mut grid);
        let up_right = access_grid(xy(index.0 + 1, index.1 + 1, size), &mut grid);
        let down_left = access_grid(xy(index.0 - 1, index.1 - 1, size), &mut grid);

        let mut acc = up + down + left + right + up_left + down_right + up_right + down_left;

        if acc == 0 {
            acc = 1;
        }   

        grid[xy(index.0, index.1, size)] = acc;

        let next_index = current_direction.next_dir().as_vertex(index);
        if access_grid(xy(next_index.0, next_index.1, size), &mut grid) == 0 {
            current_direction = current_direction.next_dir();
        }

        index = current_direction.as_vertex(index);

        if acc > input {
            return acc;
        }
    }
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
            break;
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::shortest_distance;
    use super::shortest_distance2;

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
