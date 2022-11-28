fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

struct KeyPad {
    cur_button: usize, // idx of current button
    buttons: [usize; 9],
}

impl KeyPad {
    fn new() -> Self {
        Self { cur_button: 4, buttons: [1, 2, 3, 4, 5, 6, 7, 8, 9] }
    }

    fn do_move(&mut self, m: char) {
        self.cur_button = match m {
            'U' => if self.cur_button < 3 { self.cur_button } else { self.cur_button - 3 },
            'R' => if [2, 5, 8].contains(&self.cur_button) { self.cur_button } else { self.cur_button + 1 },
            'D' => if self.cur_button >= 6 { self.cur_button } else { self.cur_button + 3 },
            'L' => if [0, 3, 6].contains(&self.cur_button) { self.cur_button } else { self.cur_button - 1},
            _ => panic!("invalid direction")
        }
    }

    fn determine_pass_code(&mut self, instructions: Vec<Vec<char>>) -> usize {
        let mut pass_code = 0;
        for instruction in instructions {
            pass_code *= 10;
            for m in instruction {
                self.do_move(m);
            }
            pass_code += self.buttons[self.cur_button];
        }

        pass_code
    }
}

struct AlphaNumKeyPad {
    cur_button: (usize, usize), 
    buttons: Vec<Option<char>>,
    dim: (usize, usize), // col, row
}

impl AlphaNumKeyPad {
    fn new() -> Self {
        let cur_button = (0, 2);
        let buttons = vec![
            None, None, Some('1'), None, None,
            None, Some('2'), Some('3'), Some('4'), None,
            Some('5'), Some('6'), Some('7'), Some('8'), Some('9'),
            None, Some('A'), Some('B'), Some('C'), None,
            None, None, Some('D'), None, None,
        ];
        Self { cur_button, buttons,  dim: (5, 5) }
    }

    fn get(&self, col: isize, row: isize ) -> Option<char> {
        if col >= self.dim.0 as isize || row >= self.dim.1 as isize || col < 0 || row < 0 {
            return None;
        }

        let idx = self.idx(col as usize, row as usize);
        match self.buttons.get(idx) {
            Some(&Some(c)) => Some(c),
            _ => None
        }
    }

    fn idx(&self, col: usize, row: usize) -> usize {
        self.dim.1 * row + col
    }

    fn do_move(&mut self, m: char) {
        let (col, row) = match m {
            'U' => (self.cur_button.0 as isize, self.cur_button.1 as isize - 1),
            'R' => (self.cur_button.0 as isize + 1, self.cur_button.1 as isize),
            'D' => (self.cur_button.0 as isize, self.cur_button.1 as isize + 1),
            'L' => (self.cur_button.0 as isize - 1, self.cur_button.1 as isize),
            _ => panic!("invalid direction")
        };

        if let Some(_) = self.get(col, row) {
            self.cur_button = (col as usize, row as usize);
        }
    }

    fn determine_pass_code(&mut self, instructions: Vec<Vec<char>>) -> String {
        let mut pass_code = String::new();
        for instruction in instructions {
            for m in instruction {
                self.do_move(m);
            }
            pass_code.push(self.get(self.cur_button.0 as isize, self.cur_button.1 as isize).unwrap());
        }

        pass_code
    }
}

pub fn get_solution_1() -> usize {
    let instructions = parse(include_str!("../data/d02.txt"));
    let mut key_pad = KeyPad::new();
    key_pad.determine_pass_code(instructions)
}

pub fn get_solution_2() -> String {
    let instructions = parse(include_str!("../data/d02.txt"));
    let mut key_pad = AlphaNumKeyPad::new();
    key_pad.determine_pass_code(instructions) 
}