pub fn get_solution_1() -> usize {
    for a in 0.. {
        if find_a(a) {
            return a;
        }
    }
    unreachable!();
}

fn run_program(n: isize) {
    let mut a = n + 15 * 170;
    loop {
        a /= 2;
        println!("{}", a % 2);
        if a == 0 {
            a = n + 15 * 170;
        }
    }
}

fn find_a(mut a: usize) -> bool {
    a += 170 * 15;
    let mut is_odd = a % 2 == 1;
    while a > 0 {
        if a % 2 == 1 && !is_odd || a % 2 == 0 && is_odd {
            return false;
        } else {
            a /= 2;
            is_odd = !is_odd;
        }
    }
    return true;
}