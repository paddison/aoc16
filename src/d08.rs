use std::fmt::Display;

#[derive(Debug)]
struct Screen<const D: usize> {
    pixels: [bool; D],
    dim: (usize, usize), // (cols, rows)
}

impl<const D: usize> Screen<D> {
    fn new(cols: usize, rows: usize) -> Self {
        assert_eq!(rows * cols, D);
        Self { pixels: [false; D], dim: (cols, rows)}
    }

    fn rotate_row(&mut self, row: usize, amount: usize) {
        let mut new_pixels = self.pixels;

        for col in 0..self.dim.0 {
            new_pixels[self.idx((col + amount) % self.dim.0, row)] = self.pixels[self.idx(col, row)];
        }

        self.pixels = new_pixels;
    }

    fn rotate_col(&mut self, col: usize, amount: usize) {
        let mut new_pixels = self.pixels;

        for row in 0..self.dim.1 {
            new_pixels[self.idx(col, (row + amount) % self.dim.1)] = self.pixels[self.idx(col, row)];
        }

        self.pixels = new_pixels;
    }

    fn create_rect(&mut self, cols: usize, rows: usize) {
        for row in 0..rows {
            for col in 0..cols {
                self.pixels[self.idx(col, row)] = true;
            }
        }   
    }

    #[inline(always)]
    fn idx(&self, col: usize, row: usize) -> usize {
        self.dim.0 * row + col
    }

    fn handle_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::RotateRow(row, amount) => self.rotate_row(row, amount),
            Instruction::RotateCol(col, amount) => self.rotate_col(col, amount),
            Instruction::CreateRect(cols, rows) => self.create_rect(cols, rows),
        }
    }

    fn count_lit_pixels(&self) -> usize {
        self.pixels.iter().filter(|p| **p).count()
    }
}

impl <const D: usize> Display for Screen<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.dim.1 {
            let mut row_disp = String::new();
            for col in 0..self.dim.0 {
                match self.pixels[self.idx(col, row)] {
                    true => row_disp.push('#'),
                    false => row_disp.push('.'),
                }
            }
            let _ = writeln!(f, "{}", row_disp);
        }

        Ok(())
    }
}

enum Instruction {
    RotateRow(usize, usize), // row, amount
    RotateCol(usize, usize), // col, amount
    CreateRect(usize, usize) // col, row
}

impl From<Vec<&str>> for Instruction {
    fn from(parts: Vec<&str>) -> Self {
        match parts[0] {
            "rect" => {
                let idx = parts[1].find('x').unwrap();
                Self::CreateRect(
                    parts[1][..idx].parse::<usize>().unwrap(), 
                    parts[1][idx + 1..].parse::<usize>().unwrap()
                )
            },
            "rotate" => match (parts[2][2..].parse::<usize>(), parts[4].parse::<usize>()) {
                (Ok(idx), Ok(amount)) => if parts[1] == "row" {
                        Self::RotateRow(idx, amount) 
                    } else {
                        Self::RotateCol(idx, amount)
                    },
                _ => unreachable!(),
                }, 
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines()
         .map(|line| Instruction::from(line.split_ascii_whitespace().collect::<Vec<&str>>()))
         .collect()
}

pub fn get_solution_1() -> usize {
    let instructions = parse(include_str!("../data/d08.txt"));
    let mut screen = Screen::<300>::new(50, 6);
    for instruction in instructions {
        screen.handle_instruction(instruction);
    }
    screen.count_lit_pixels()
}

pub fn get_solution_2() -> &'static str {
    let instructions = parse(include_str!("../data/d08.txt"));
    let mut screen = Screen::<300>::new(50, 6);
    for instruction in instructions {
        screen.handle_instruction(instruction);
    }
    // println!("{}", screen); 
    "AFBUPZBJPS" // what screen printed
}
