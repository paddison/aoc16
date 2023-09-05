static TEST: &str = include_str!("../data/d20_test.txt");
static INPUT: &str = include_str!("../data/d20.txt");

pub fn get_solution_1() -> usize {
    build_overlapping(parse_input(INPUT))[0].1 + 1
}

pub fn get_solution_2() -> usize {
    count_whitelist(build_overlapping(parse_input(INPUT)))
}

fn parse_input(inp: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    for l in inp.lines() {
        let split = l.split('-').map(|n| usize::from_str_radix(n, 10).unwrap()).collect::<Vec<usize>>();
        ranges.push((split[0], split[1]));
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    ranges
}

fn build_overlapping(ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut overlapping = Vec::from([ranges[0]]);
    for r in ranges.into_iter().skip(1) {
        // they don't overlap
        let last = overlapping.last_mut().unwrap();
        if last.1 + 1 < r.0 {
            overlapping.push(r)
        // check if they overlap
        } else if last.1 < r.1 {
            last.1 = r.1; 
        }
    }

    overlapping
}

fn count_whitelist(overlapping: Vec<(usize, usize)>) -> usize {
    let mut count = 0;
    for w in &mut overlapping.windows(2) {
        let (r, l) = (w[0], w[1]);
        count += l.0 - r.1 - 1;
    }
    count
}
