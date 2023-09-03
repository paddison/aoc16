use md5;
use std::collections::VecDeque;

static PC1: &str = "hijkl";
static PC2: &str = "ihgpwlah";
static PC3: &str = "kglvqrro";
static PC4: &str = "ulqzkmiv";
static INPUT: &str = "dmypynyp";

pub fn get_solution_1() -> String {
    bfs(State::new(), INPUT)
}

pub fn get_solution_2() -> usize {
    bfs_longest(State::new(), INPUT)
}

struct State {
    pos: (usize, usize),
    steps: String,
}

impl State {
    fn new() -> Self {
        Self { pos: (0, 0), steps: String::new() }
    }

    fn is_goal(&self) -> bool {
        self.pos == (3, 3)
    }

    fn next(&self, pc: &str) -> Vec<Self> {
        let dirs = ['U', 'D', 'L', 'R'];
        let mut next = Vec::new(); 
        let hash = md5::compute(format!("{}{}", pc, self.steps));
        for (i, c) in format!("{:x}", hash).chars().take(4).enumerate() {
            match c {
                'b' | 'c' | 'd' | 'e' | 'f' => if let Some(state) = self.step(dirs[i]) {
                    next.push(state)
                },
                _ => continue,
            }
        }

        next
    }

    fn step(&self, dir: char) -> Option<Self> {
        let pos = match dir {
            'U' => if self.pos.1 == 0 { return None } else { (self.pos.0, self.pos.1 - 1) },
            'D' => if self.pos.1 == 3 { return None } else { (self.pos.0, self.pos.1 + 1) },
            'L' =>if self.pos.0 == 0 { return None } else { (self.pos.0 - 1, self.pos.1)},
            'R' => if self.pos.0 == 3 { return None } else { (self.pos.0 + 1, self.pos.1) },
            _ => unreachable!(),
        };

        Some(Self { pos, steps: format!("{}{}", self.steps, dir) })
    }


}


fn bfs(start: State, pc: &str) -> String {
    let mut queue =  VecDeque::from([start]);
    while let Some(state) = queue.pop_front() {
        if state.is_goal() {
            return state.steps;
        }
        for next in state.next(pc) {
            queue.push_back(next);
        }
    }
    unreachable!();
}

fn bfs_longest(start: State, pc: &str) -> usize {
    let mut longest = 0;
    let mut queue =  VecDeque::from([start]);
    while let Some(state) = queue.pop_front() {
        if state.is_goal() {
            longest = state.steps.len();
            continue;
        }
        for next in state.next(pc) {
            queue.push_back(next);
        }
    }

    longest
}
