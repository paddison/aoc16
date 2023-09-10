use std::collections::{HashMap, HashSet, VecDeque};
// determine shortest path between each number pair with bfs
// then try to minimize the total path

static INPUT: &str = include_str!("../data/d24.txt");
static _TEST: &str = include_str!("../data/d24_test.txt");

pub fn get_solution_1() -> usize {
    find_min_path(build_adj_matrix(find_distances(parse_input(INPUT))), false)
}

pub fn get_solution_2() -> usize {
    find_min_path(build_adj_matrix(find_distances(parse_input(INPUT))), true)
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, HashMap<char, (usize, usize)>) {
    let mut nums = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, tile) in line.chars().enumerate() {
            match tile {
                '.' => continue,
                '#' => continue,
                n => nums.insert(n, (row, col)),
            };
        }
    }
    (input.lines().map(|line| line.chars().collect()).collect(), nums)
}

fn bfs((row, col): (usize, usize), to_find: &HashSet<char>, map: &[Vec<char>]) -> Vec<(char, usize)> {
    let mut distances = Vec::new();
    let mut visited = HashSet::from([(row, col)]);
    let mut queue = VecDeque::from([(row, col, 0)]);
    while let Some((row, col, mut steps)) = queue.pop_front() {
        if to_find.len() == distances.len() {
            break;
        }
        steps += 1;
        // find neighbors
        for (next_row, next_col) in [(row - 1, col), (row, col - 1), (row, col + 1), (row + 1, col)] {
            if visited.contains(&(next_row, next_col)) {
                continue;
            }
            match map[next_row][next_col] {
                '#' => continue,
                '.' => (),
                n => {
                    if to_find.contains(&n) {
                        distances.push((n, steps))
                    }
                }
            }
            visited.insert((next_row, next_col));
            queue.push_back((next_row, next_col, steps));
        }
    }
    distances
}

fn find_distances((map, chars): (Vec<Vec<char>>, HashMap<char, (usize, usize)>)) -> HashMap<char, Vec<(char, usize)>>{
    let mut distances = HashMap::new();
    let mut to_find = chars.keys().copied().collect::<HashSet<_>>();
    for (c, pos) in chars {
        to_find.remove(&c);
        let char_dists = bfs(pos, &to_find, map.as_slice());
        distances.insert(c, char_dists);
    }
    distances
} 

fn build_adj_matrix(distances: HashMap<char, Vec<(char, usize)>>) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![0; distances.len()]; distances.len()];
    for (v, ns) in distances {
        let v = v.to_digit(10).unwrap() as usize;
        for (n, dist) in ns {
            let n = n.to_digit(10).unwrap() as usize;
            graph[v][n] = dist;
            graph[n][v] = dist;
        }
    } 
    graph
}

fn find_min_path(graph: Vec<Vec<usize>>, is_p2: bool) -> usize {
    let mut paths = Vec::new();
    determine_path(0, graph, &mut paths, 0, HashSet::new(), is_p2);
    *paths.iter().min().unwrap()
}

fn determine_path(cur: usize, mut graph: Vec<Vec<usize>>, paths: &mut Vec<usize>, total_cost: usize, visited: HashSet<usize>, is_p2: bool) {
    let neighbors = graph[cur].iter().copied().enumerate().filter(|(n, _)| !visited.contains(n)).collect::<Vec<_>>();
    if neighbors.len() == 0 {
        if is_p2 {
            paths.push(total_cost + graph[cur][0])
        } else {
            paths.push(total_cost)
        }
    } else {
        for (n, cost) in neighbors {
            let mut visited = visited.clone();
            visited.insert(n);
            determine_path(n, graph.clone(), paths, total_cost + cost, visited, is_p2);
        }
    }
}
