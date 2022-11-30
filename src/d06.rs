use std::collections::{HashMap, hash_map::Iter};

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn count_letters(lines: Vec<&str>) -> Vec<HashMap<char, usize>> {
    let mut maps = vec![HashMap::new(); lines[0].len()];

    for line in lines {
        for (i, letter) in line.chars().enumerate() {
            let map = maps.get_mut(i).unwrap();
            let entry = map.entry(letter).or_insert(0);
            *entry += 1;
        }
    }

    maps
}

fn get_msg<F>(maps: Vec<HashMap<char, usize>>, f: F) -> String 
where F: Fn(Iter<'_, char, usize>) -> Option<(&char, &usize)>
{
    let mut msg = String::new();
    for map in maps {
        let ch = f(map.iter()).map(|(k, _)| k).unwrap();
        msg.push(*ch);
    }

    msg
}

fn find_max(map_iter: Iter<'_, char, usize>) -> Option<(&char, &usize)> {
    map_iter.max_by(|(_, a), (_, b)| a.cmp(b))
}

fn find_min(map_iter: Iter<'_, char, usize>) -> Option<(&char, &usize)> {
    map_iter.min_by(|(_, a), (_, b)| a.cmp(b))
}

pub fn get_solution_1() -> String {
    get_msg(count_letters(parse(include_str!("../data/d06.txt"))), find_max)
}

pub fn get_solution_2() -> String {
    get_msg(count_letters(parse(include_str!("../data/d06.txt"))), find_min)
}