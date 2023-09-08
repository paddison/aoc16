static INPUT: &str = include_str!("../data/d22.txt");

type Grid<const Y: usize, const X: usize> = [[Data; Y]; X];

#[derive(Clone, Copy, Debug)]
struct Data {
    size: usize,
    used: usize,
}

impl Data {
    fn fits(&self, other: Self) -> bool {
        self.used <= other.size - other.used
    }

    fn is_empty(&self) -> bool {
        self.used == 0
    }
}   

pub fn get_solution_1() -> usize {
    count_viable_pairs(parse_input::<33, 31>(INPUT))
}

pub fn get_solution_2() -> usize {
    determine_steps::<33, 31>(find_empty_node(&parse_input::<33, 31>(INPUT)))
}

fn parse_input<const X: usize, const Y: usize>(inp: &str) -> Grid<X, Y> {
    const D: Data = Data{ size: 0, used: 0 };
    let mut data = [[D; X]; Y];

    // skip the header
    let mut lines = inp.lines().skip(2);

    // fill in the data
    for x in 0..X {
        for y in 0..Y {
            let line = lines.next().unwrap().split_whitespace().collect::<Vec<&str>>();
            let (size, used) = match &line[1..3] {
                &[size, used] => (
                    usize::from_str_radix(size.trim_end_matches('T'), 10).unwrap(),
                    usize::from_str_radix(used.trim_end_matches('T'), 10).unwrap()
                ),
                _ => unreachable!()
            };
            data[y][x] = Data { size, used };
        }
    }
    
    data
}

fn count_viable_pairs<const X: usize, const Y: usize>(grid: Grid<X, Y>) -> usize {
    let mut viable_pairs = 0;
    for x in 0..X {
        for y in 0..Y {
            let data = grid[y][x];
            if data.is_empty() {
                continue;
            }
            for x2 in 0..X {
                for y2 in 0..Y {
                    if x == x2 && y == y2 {
                        continue;
                    }
                    let other = grid[y2][x2];
                    if data.fits(other) {
                        viable_pairs += 1;
                    }
                }
            }
        }
    }

    viable_pairs
}

fn find_empty_node<const X: usize, const Y: usize>(grid: &Grid<X, Y>) -> (usize, usize) {
    for x in 0..X {
        for y in 0..Y {
            if grid[y][x].is_empty() {
                return (x, y)
            }
        }
    }
    unreachable!()
}

fn determine_steps<const X: usize, const Y: usize>((x, y): (usize, usize)) -> usize {
    // empty starts at (3, 28)
    // the big data are all at y == 20
    // the one free spot where we can move is at x0 y20
    // move empty to the left -> 3 steps
    // move empty up 28 steps
    // move empty to the right -> 32 steps
    // it takes 5 steps to move the data from a cell one step to the left
    // we need to do this a total of 31 times
    // therefore we need  empty.x + empty.y + X - 1 + (X - 2) * 5 steps
    return x + y + X - 1 + (X - 2) * 5;
}