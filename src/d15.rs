static TEST: [(usize, usize); 2] = [(5, 4), (2, 1)];
static INPUT: [(usize, usize); 6] = [(13, 10), (17, 15), (19, 17), (7, 1), (5, 0), (3, 1)];
static INPUT_2: [(usize, usize); 7] = [(13, 10), (17, 15), (19, 17), (7, 1), (5, 0), (3, 1), (11, 0)];

pub fn get_solution_1() -> usize {
    find_t(&INPUT)
}

pub fn get_solution_2() -> usize {
    find_t(&INPUT_2)
}

fn find_t(input: &[(usize, usize)]) -> usize {
    for t in 0.. {
        if input.iter().enumerate().all(move |(n, (pos, start))| (start + (1 + n) + t) % pos == 0) {
            return t;
        }
    }
    unreachable!();
}
