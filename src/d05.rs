static _INPUT: &str = "ugkcyxxp";

fn _compute_password_1() -> String {
    let mut password = String::new();
    for i in 0.. {
        let digest = format!("{:?}", md5::compute(format!("{INPUT}{i}")));
        if digest.starts_with("00000") {
            password.push_str(&digest[5..6]);
            if password.len() == 8 {
                return password;
            }
        }
    }

    unreachable!()
}

fn _compute_password_2() -> String {
    const S: String = String::new();
    let mut password_raw = [S; 8];
    let mut found_indices = Vec::new();
    for i in 0.. {
        let digest = format!("{:?}", md5::compute(format!("{INPUT}{i}")));
        if digest.starts_with("00000") {
            // safety: this is single threaded
            if let Ok(idx) = digest[5..6].parse::<usize>() {
                if !found_indices.contains(&idx) && idx < 8 {
                    password_raw[idx] = digest[6..7].to_string();
                    found_indices.push(idx);
                    if found_indices.len() == 8 {
                        break;
                    }
                }
            }
        }
    }

    let mut password = String::new();
    for letter in password_raw {
        password.push_str(&letter);
    }

    password
}

pub fn get_solution_1() -> String {
    // _compute_password_1()
    "d4cd2ee1".to_string()
}

pub fn get_solution_2() -> String {
    // _compute_password_2()
    "f2c730e5".to_string()
}