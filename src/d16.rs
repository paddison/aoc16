pub fn get_solution_1() -> String {
    build_checksum(build_data("10001110011110000", 272), 272)
}

pub fn get_solution_2() -> String {
    build_checksum(build_data("10001110011110000", 35651584), 35651584)
}

fn dragon_curve(n: String) -> String {
    let reversed = n.chars().rev().map(|c| if c == '0' { '1' } else { '0' }).collect::<String>();
    format!("{n}{}{reversed}", 0)
}

fn build_data(n: &str, len: usize) -> String {
    let mut data = n.to_string();
    loop {
        if data.len() >= len {
            break data;
        }
        data = dragon_curve(data);
    }
}

fn checksum(data: String) -> String {
    let mut checksum = String::new();
    for (l, r) in data.chars().step_by(2).zip(data.chars().skip(1).step_by(2)) {
        if l == r {
            checksum.push('1');
        } else {
            checksum.push('0');
        }
    }
    checksum
}

fn build_checksum(mut data: String, len: usize) -> String {
    data = data.chars().take(len).collect();
    loop {
        if data.len() % 2 == 1 {
            break data;
        }
        data = checksum(data);
    }
}

#[test]
fn d16() {
    println!("{}", build_checksum(build_data("10001110011110000", 35651584), 35651584));
}
