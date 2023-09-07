static INPUT:  &str = include_str!("../data/d21.txt"); 
static TEST: &str = include_str!("../data/d21_test.txt");
static STRING: &str = "abcdefgh";
static PW: &str = "fbgdceah";

#[derive(Clone, Copy)]
enum Instr {
    SwapP(usize, usize),
    SwapL(char, char),
    RotateP(&'static str, usize),
    RotateL(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

pub fn get_solution_1() -> String {
    execute(parse_input(INPUT), STRING)
}

pub fn get_solution_2() -> String {
    let instructions = parse_input(INPUT);
    let mut perms = Vec::new();
    let letters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    permute(letters, Vec::new(), &mut perms);
    for perm in perms {
        if execute(instructions.clone(), &perm).as_str() == PW {
            return perm;
        }
    }
    unreachable!();
}

fn parse_input(input: &'static str) -> Vec<Instr> {
    use Instr as I;

    let mut instructions = Vec::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let instr = match &parts[..] {
            &["swap", "position", x, _, _, y] => I::SwapP(usize::from_str_radix(x, 10).unwrap(), usize::from_str_radix(y, 10).unwrap()),
            &["swap", "letter", x, _, _, y] => I::SwapL(x.chars().next().unwrap(), y.chars().next().unwrap()),
            &["rotate", "based", _, _, _, _, x] => I::RotateL(x.chars().next().unwrap()),
            &["rotate", dir, x, ..] => I::RotateP(dir, usize::from_str_radix(x, 10).unwrap()),
            &["reverse", _, x, _, y] => I::Reverse(usize::from_str_radix(x, 10).unwrap(), usize::from_str_radix(y, 10).unwrap()),
            &["move", _, x, _, _, y] => I::Move(usize::from_str_radix(x, 10).unwrap(), usize::from_str_radix(y, 10).unwrap()),
            _ => unreachable!(),
        };
        instructions.push(instr);
    }

    instructions
}

fn execute(instructions: Vec<Instr>, input: &str) -> String {
    use Instr as I;

    let mut input = input.chars().collect::<Vec<_>>();
    for instr in instructions {
        match instr {
            I::SwapP(x, y) => swap_p(&mut input, x, y),
            I::SwapL(x, y) => swap_l(&mut input, x, y),
            I::RotateP(dir, n) => rotate_p(&mut input, dir, n),
            I::RotateL(x) => rotate_l(&mut input, x),
            I::Reverse(start, end) => reverse(&mut input, start, end),
            I::Move(x, y) => move_l(&mut input, x, y),
        }
    }

    input.iter().collect::<String>()
}

fn swap_p(string: &mut [char], x: usize, y: usize) {
    string.swap(x, y);
}

fn swap_l(string: &mut [char], x: char, y: char) {
    let x_pos = string.iter().position(|l| x == *l).unwrap();
    let y_pos = string.iter().position(|l| y == *l).unwrap();
    swap_p(string, x_pos, y_pos);
}

fn rotate_p(string: &mut [char], dir: &'static str, n: usize) {
    match dir {
        "left" => string.rotate_left(n),
        _ => string.rotate_right(n),
    }
}

fn rotate_l(string: &mut [char], x: char) {
    let pos = string.iter().position(|l| x == *l).unwrap();
    string.rotate_right(1);
    string.rotate_right(pos);
    if pos >= 4 {
        string.rotate_right(1);
    }
}

fn reverse(string: &mut [char], start: usize, end: usize) {
    string[start..end + 1].reverse();
}

fn move_l(string: &mut Vec<char>, x: usize, y: usize) {
    let c = string.remove(x);
    if y == string.len() {
        string.push(c);
    } else {
        string.insert(y, c);
    }
}

fn permute(inp: Vec<char>, mut perm: Vec<char>, permutations: &mut Vec<String>) {
    if inp.len() == 1 {
        perm.push(inp[0]); 
        permutations.push(perm.into_iter().collect());
        
    } else {
        for c in &inp {
            let mut perm = perm.clone();
            let mut inp = inp.clone();
            perm.push(*c);
            inp.remove(inp.iter().position(|other| c == other).unwrap());
            permute(inp, perm, permutations);
        }
    }
}