type Triangle = [u64; 3];

fn parse(input: &str) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    for l in input.lines() {
        let mut t = [0; 3];
        for (idx, n) in l.split_ascii_whitespace().enumerate() {
            t[idx] = n.parse::<u64>().unwrap();
        }
        triangles.push(t);
        
    }
    triangles
}

fn parse_vertical(input: &str) -> Vec<Triangle> {
    let mut triangles = vec![[0; 3]; input.lines().count()];

    for (i, l) in input.lines().enumerate() {
        for (j, n) in l.split_ascii_whitespace().enumerate() {
            triangles[(i / 3) * 3 + j][i % 3] = n.parse::<u64>().unwrap();
        }

    }
    triangles
}

fn check_sides(t: &Triangle) -> bool {
    t[0] + t[1] > t[2] &&
    t[0] + t[2] > t[1] &&
    t[1] + t[2] > t[0]
}

fn count_triangles(triangles: Vec<Triangle>) -> usize {
    triangles.iter().filter(|t| check_sides(t)).count()
}

pub fn get_solution_1() -> usize {
    count_triangles(parse(include_str!("../data/d03.txt")))
}

pub fn get_solution_2() -> usize {
    count_triangles(parse_vertical(include_str!("../data/d03.txt")))
}