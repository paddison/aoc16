use std::collections::HashMap;

fn parse(input: &'static str) -> Vec<Room> {
    input.lines().map(|line| line.into()).collect()
}

struct Room {
    checksum: [char; 5],
    id: u32,
    name_char_count: HashMap<char, usize>,
    name: &'static str,
}

impl Room {
    fn is_real(&self) -> bool {
        for (a, b) in self.checksum.iter().zip(&self.checksum[1..]) {
            let (a_count, b_count) = match (self.name_char_count.get(a), self.name_char_count.get(b)) {
                (Some(a_count), Some(b_count)) => (a_count, b_count),
                _ => return false,
            };
            if a_count < b_count {
                return false;
            }

            if a_count == b_count && (u32::from(*a) as i32 - u32::from(*b) as i32) >= 0{
                return false;
            }
        }

        true
    }

    fn get_msg(&self) -> String {
        let mut real_name = String::new();

        for ch in self.name.chars() {
            if ch == '-' {
                real_name.push(' ')
            } else {
                real_name.push(Self::rotate(ch, self.id));
            }
        }
        
        real_name
    }

    fn rotate(ch: char, id: u32) -> char {
        let a_ascii = 97;
        let n_chars = 26;
        let ch_u32 = u32::from(ch) - a_ascii;
        let rotated = ((ch_u32 + id) % n_chars) + a_ascii;

        char::from_u32(rotated).unwrap()
    } 
}

impl From<&'static str> for Room {
    fn from(input: &'static str) -> Self {
        let checksum_delim = input.find('[').unwrap();
        let id_delim = input.rfind('-').unwrap();
        let mut checksum = ['0'; 5];
        let mut name_char_count = HashMap::new();

        input[checksum_delim + 1..checksum_delim + 6].chars().enumerate().for_each(|(i, c)| checksum[i] = c);
        let id = input[id_delim + 1..checksum_delim].parse::<u32>().unwrap();
        let name = &input[..id_delim];
        for c in input[..id_delim].chars() {
            if c == '-' {
                continue;
            }
            let entry = name_char_count.entry(c).or_insert(0);
            *entry += 1;
        }

        Self { checksum, id, name_char_count, name }
    }
}

pub fn get_solution_1() -> u32 {
    parse(include_str!("../data/d04.txt")).iter()
        .filter(|room| room.is_real())
        .fold(0, |a, b| a + b.id)
}

pub fn get_solution_2() -> u32 {
    parse(include_str!("../data/d04.txt")).iter()
        .find(|room| room.is_real() && room.get_msg().contains("north"))
        .map(|r| r.id)
        .unwrap()
}