use std::collections::{ HashSet, VecDeque };

const INPUT: usize = 1358;

pub fn get_solution_1() -> usize {
    bfs(VecDeque::from([((1, 1), 0)]), HashSet::new())
}

pub fn get_solution_2() -> usize {
    count_locations(VecDeque::from([((1, 1), 0)]), HashSet::new())
}

fn bfs(mut queue: VecDeque<((usize, usize), usize)>, mut visited: HashSet<(usize, usize)>) -> usize {
    let goal = (31, 39);
    while let Some((s, steps)) = queue.pop_front() {
        visited.insert(s);
        if s == goal { return steps; }
        next_moves(s, &visited).into_iter().for_each(|m| queue.push_back((m, steps + 1)));
    }
    unreachable!();
}

fn count_locations(mut queue: VecDeque<((usize, usize), usize)>, mut visited: HashSet<(usize, usize)>) -> usize {
    while let Some((s, steps)) = queue.pop_front() {
        visited.insert(s);
        if steps == 50 { continue; }
        next_moves(s, &visited).into_iter().for_each(|m| queue.push_back((m, steps + 1)));
    }
    visited.len()
}

fn is_open(x: usize, y: usize) -> bool {
    let n = x * x + 3 * x + 2 * x * y + y + y * y + INPUT;
    n.count_ones() % 2 == 0
}

fn next_moves((x, y): (usize, usize), visited: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    [(x, y.wrapping_sub(1)), (x.wrapping_sub(1), y), (x + 1, y), (x, y + 1)].into_iter()
        .filter(|(x2, y2)| !(*x2 == usize::MAX || *y2 == usize::MAX) && is_open(*x2, *y2) && !visited.contains(&(*x2, *y2)))
        .collect()
}
