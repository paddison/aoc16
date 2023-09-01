static TEST: &str = include_str!("../data/d12_test.txt");
static INPUT: &str = include_str!("../data/d12.txt");

fn parse_instructions(input: &'static str) -> Vec<Instr> {
    use Instr::*;
    let mut instructions = Vec::new();
    for line in input.lines() {
        let line = line.split_whitespace().collect::<Vec<_>>();
        let instr = match &line[..] {
            &["cpy", r, l] => Cpy(r.into(), l.into()),
            &["inc", v] => Inc(v.into()),
            &["dec", v] => Dec(v.into()),
            &["jnz", r, l] => Jnz(r.into(), l.into()),
            _ => unreachable!()
        };
        instructions.push(instr);
    }

    instructions
}

pub(crate) fn get_solution_1() -> isize {
    let mut cpu = Cpu::new();
    cpu.execute_instructions(parse_instructions(INPUT));
    cpu.a
}

pub(crate) fn get_solution_2() -> isize {
    let mut cpu = Cpu::new();
    cpu.c = 1;
    cpu.execute_instructions(parse_instructions(INPUT));
    cpu.a
}

#[derive(Debug)]
struct Cpu {
    a: isize,
    b: isize,
    c: isize,
    d: isize,
}

impl Cpu {
    fn new() -> Self {
        Self { a: 0, b: 0, c: 0, d: 0 }
    }

    // returns the next position of the program counter
    fn execute(&mut self, instr: Instr) -> isize {
        use Instr::*;

        match instr {
            Cpy(r, l) => self.cpy(r, l),
            Inc(reg) => self.inc(reg),
            Dec(reg) => self.dec(reg),
            Jnz(r, l) => self.jnz(r, l),
        }
    }

    fn execute_instructions(&mut self, instructions: Vec<Instr>) {
        let mut pc = 0;
        while let Some(instr) = instructions.get(pc) {
            pc = (pc as isize + self.execute(*instr)) as usize;
        }

    }

    fn cpy(&mut self, r: Type, l: Type) -> isize {
        let r = match r {
            Type::Reg(name) => *self.get_register(name),
            Type::Num(n) => n

        };
        *self.get_register(l.unwrap_reg()) = r;
        1
    }

    fn inc(&mut self, reg: Type) -> isize {
        *self.get_register(reg.unwrap_reg()) += 1;
        1
    }

    fn dec(&mut self, reg: Type) -> isize {
        *self.get_register(reg.unwrap_reg()) -= 1;
        1
    }

    fn jnz(&mut self, r: Type, l: Type) -> isize {
        let r = match r {
            Type::Reg(name) => *self.get_register(name),
            Type::Num(n) => n,
        };
        if r != 0 {
            l.unwrap_num()
        } else {
            1
        }
    }

    fn get_register(&mut self, name: &str) -> &mut isize {
        match name {
            "a" => &mut self.a,
            "b" => &mut self.b,
            "c" => &mut self.c,
            "d" => &mut self.d,
            _ => unreachable!(),
        }
    }

}

#[derive(Clone, Copy)]
enum Instr {
    Cpy(Type, Type),
    Inc(Type),
    Dec(Type),
    Jnz(Type, Type),
}

#[derive(Clone, Copy)]
enum Type {
    Reg(&'static str),
    Num(isize),
}

impl Type {
    fn unwrap_reg(self) -> &'static str {
        match self {
            Self::Reg(name) => name,
            _ => panic!("called unwrap_reg on num type"),
        }
    }
    
    fn unwrap_num(self) -> isize {
        match self {
            Self::Num(n) => n,
            _ => panic!("called unwrap_num on reg type"),
        }
    }
}

impl From<&'static str> for Type {
    fn from(input: &'static str) -> Self {
        match isize::from_str_radix(input, 10) {
            Ok(n) => Self::Num(n),
            Err(_) => {
                assert!(input.len() == 1);
                Self::Reg(input)
            },
        }
    }
}
