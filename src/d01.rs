fn parse(input: &str) -> Vec<Turn> {
    input.split_ascii_whitespace()
         .map(|turn| turn.into())
         .collect()
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left(isize),
    Right(isize),
}

impl From<&str> for Turn {
    fn from(step: &str) -> Self {
        let n_steps: isize = step.trim_end_matches(',')[1..].parse().unwrap();
        if step.starts_with('L') {
            Turn::Left(n_steps)
        } else {
            Turn::Right(n_steps)
        }
    }
}

struct Pos {
    x: isize,
    y: isize,
    dir: usize,
}

impl Pos {
    fn new() -> Self {
        Self { x: 0, y: 0, dir: 0 }
    }

    fn update_pos(&mut self, n_steps: isize) {
        match self.dir  {
            0 => self.y += n_steps, // north, south
            1 => self.x += n_steps, // east, west
            2 => self.y -= n_steps,
            3 => self.x -= n_steps,
            _ => panic!("invalid direction")
        }
    }

    fn update_dir(&mut self, turn: Turn) -> isize {
        let (modifier, n_steps) = match turn {
            Turn::Right(n_steps) =>  (1, n_steps),
            Turn::Left(n_steps) => (3, n_steps),
        };

        self.dir = (self.dir + modifier) % 4;

        n_steps
    }

    fn get_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }

    fn walk(&mut self, all_turns: Vec<Turn>) {
        for step in all_turns {
            let n_steps = self.update_dir(step);
            self.update_pos(n_steps);
        } 
    }

    fn walk_until_visited_twice(&mut self, all_turns: Vec<Turn>) {
        let mut visited = Vec::new();
        for turn in all_turns {
            for _ in 0..self.update_dir(turn) {
                visited.push((self.x, self.y));
                self.update_pos(1);
                if visited.contains(&(self.x, self.y)) {
                    return;
                }
            }
        }
    }
}

pub fn get_solution_1() -> isize {
    let all_steps = parse(include_str!("../data/d01.txt"));
    let mut pos = Pos::new();
    pos.walk(all_steps);
    pos.get_distance()
}

pub fn get_solution_2() -> isize {
    let all_steps = parse(include_str!("../data/d01.txt"));
    let mut pos = Pos::new();
    pos.walk_until_visited_twice(all_steps);
    pos.get_distance()
}