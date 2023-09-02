use md5;

static _TEST: &str = "abc";
static INPUT: &str = "jlmsuwbz";

pub fn get_solution_1() -> usize {
    count_hashes(INPUT, hash)
}

pub fn get_solution_2() -> usize {
    count_hashes(INPUT, key_stretching)
}

fn count_hashes(salt: &str, compute: fn(&str, usize) -> String) -> usize {
    const EMPTY: String = String::new();
    let mut hashes = [EMPTY; 1001];
    for i in 0..1000 {
        hashes[i] = compute(salt, i);
    }
    let mut n_keys = 0;

    for i in 0.. {
        // compute the next hash
        hashes[(i + 1000) % 1001] = compute(salt, i + 1000);
        if let Some(c) = contains_three(&hashes[i % 1001]) {
            for j in i + 1..i + 1000 {
                if contains_five(&hashes[j % 1001], c) {
                    n_keys += 1;
                    break;
                }
            }
        }
        if n_keys == 64 {
            return i
        }
    }
    unreachable!();
}

fn hash(inp: &str, i: usize) -> String {
    format!("{:x}", md5::compute(format!("{}{}", inp, i)))
}

fn contains_three(hash: &str) -> Option<char> {
    let mut chars = hash.chars();
    let (mut first, mut second) = (chars.next().unwrap(), chars.next().unwrap());
    for third in chars {
        if first == second && second == third {
            return Some(first);
        }
        first = second; second = third;
    }
    None
}

fn contains_five(hash: &str, i: char) -> bool {
    let mut chars = hash.chars();
    let (mut a, mut b, mut c, mut d) = (chars.next().unwrap(), chars.next().unwrap(), chars.next().unwrap(), chars.next().unwrap());
    for e in chars {
        if i == a && i == b && i == c && i == d && i == e {
            return true;
        }
        a = b; b = c; c = d; d = e;
    }
    false
}

fn key_stretching(salt: &str, i: usize) -> String {
    let mut hash = hash(salt, i);
    for _ in 0..2016 {
        hash = format!("{:x}", md5::compute(hash));
    }
    hash
}
