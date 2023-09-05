pub fn get_solution_1() -> usize {
    find_elf_1(3012210)
}

pub fn get_solution_2() -> usize {
    find_elf_2(3012210) + 1
}

fn find_elf_1(n: usize) -> usize {
    (n - 2_usize.pow(n.ilog2()) as usize) * 2 + 1
}

fn find_elf_2(n: usize) -> usize {
    let mut reset = 2;
    while reset * 3 - 2 <= n {
        reset = reset * 3 - 2;
    }
    if n <= reset * 2 - 2 {
        return n - reset;
    } else {
        return (n - (reset * 2 - 2)) * 2;
    }
}
