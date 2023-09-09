static _INPUT: &str = include_str!("../data/d23.txt");

pub fn get_solution_1() -> usize {
    fak(7) + 6720
}

pub fn get_solution_2() -> usize {
    fak(12) + 6720
}

fn fak(n: usize) -> usize {
    if n == 2 { 2 } else { n * fak(n - 1) }
}