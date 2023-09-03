use std::hash::BuildHasher;

static INPUT: &str = ".^^^^^.^^.^^^.^...^..^^.^.^..^^^^^^^^^^..^...^^.^..^^^^..^^^^...^.^.^^^^^^^^....^..^^^^^^.^^^.^^^.^^";

const T: char = '^';
const S: char = '.';

pub fn get_solution_1() -> usize {
    doo()
    //count_safes(build_map(parse_input(INPUT), 40))
}

pub fn get_solution_2() -> usize {
    count_safes(build_map(parse_input(INPUT), 400000))
}

fn determine_tile(upper: &[char]) -> char {
    match upper {
        [T, T, S] | [S, T, T] | [T, S, S] | [S, S, T] => T,
        _ => S,
    }
}

// add one tile padding on both sides
fn parse_input(inp: &str) -> Vec<char> {
    let mut row = vec![S];
    row.extend(inp.chars());
    row.push(S);
    row
}

fn make_next_row(row: &[char]) -> Vec<char> {
    let mut next = vec![S];
    next.extend(row.windows(3).map(|tiles| determine_tile(tiles)));
    next.push(S);
    next
}

fn build_map(mut next: Vec<char>, length: usize) -> Vec<Vec<char>> {
    let mut map = vec![next];
    for i in 0..length - 1 {
        next = make_next_row(&map[i]);
        map.push(next);
    }
    map
}

fn count_safes(map: Vec<Vec<char>>) -> usize {
    map.into_iter()
        .map(|r| r[1..r.len() - 1].into_iter().filter(|t| **t == S).count())
        .sum::<usize>()
}

fn doo() -> usize {
    let inp = parse_input(INPUT);
    build_map_2(inp, 40)
}

fn make_next_row_2(row: &[char], c: &mut usize) -> Vec<char> {
    let mut next = vec![S];
    next.extend(row.windows(3).map(|tiles| {
        let t = determine_tile(tiles);
        if t == S { *c += 1; } 
        t
    }));
    next.push(S);
    next
}

fn build_map_2(mut next: Vec<char>, length: usize) -> usize {
    let mut c = next.iter().filter(|t| **t == S).count() - 2;
    for _ in 0..length - 1 {
        next = make_next_row_2(&next, &mut c)  
    }
    c
}
