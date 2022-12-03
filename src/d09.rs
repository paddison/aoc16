use std::fmt::Write;

fn parse(input: String) -> Vec<Entry> {
    let mut entries = Vec::new();
    let mut idx = 0; 
    loop {
        match Entry::parse(input[idx..].to_string()) {
            (entry, Some(new_idx)) => {
                entries.push(entry);
                idx += new_idx;
            },
            (entry, None) => break entries.push(entry), 
        }
    }

    entries
}

#[derive(Debug)]
struct Entry {
    marker: [usize; 2],
    chars: String,
}

impl Entry {
    fn parse(input: String) -> (Self, Option<usize>) {
        // handle edge cases, where the word does'nt start with a marker (should be impossible in actual puzzle input)
        let marker_end = match input.find('(') {
            Some(idx) if idx > 0 => return (Self { marker: [0, 0], chars: input[..idx].to_string()}, Some(idx)),
            None => return (Self { marker: [0, 0], chars: input }, None),
            Some(_) => input.find(')').unwrap(),
        }; 
        
        let mut marker = [0; 2];
        input[1..marker_end].split('x').enumerate().for_each(|(i, n)| marker[i] = n.parse::<usize>().unwrap());
        let chars = input[marker_end + 1..marker_end + 1 + marker[0]].to_string();

        (Self { marker, chars}, Some(marker_end + 1 + marker[0])) 
    }

    fn decompress(&self) -> String {
        let mut decompressed_str = String::new();
        let end = std::cmp::min(self.marker[0], self.chars.len());
        for _ in 0..self.marker[1] {
            let _ = write!(decompressed_str, "{}", &self.chars[..end]);
        }

        if end <= self.chars.len() {
            let _ = write!(decompressed_str, "{}", &self.chars[end..]);
        }

        decompressed_str
    }
}

fn decompress_all(entries: Vec<Entry>) -> String {
    let mut decompressed_str = String::new();

    for entry in entries {
        let _ = write!(decompressed_str, "{}", entry.decompress());
    }

    decompressed_str
}

fn is_decompressed(input: &str) -> bool {
    !input.contains('(')
}

fn decompress_p2(input: &str) -> (Option<usize>, usize) { // returns next_idx, string_len
    if let Some(0) = input.find('(') {
        let end = input.find(')').unwrap();
        let mut marker = [0; 2];
        input[1..end].split('x').enumerate().for_each(|(i, n)| marker[i] = n.parse::<usize>().unwrap());
        let (idx_opt, length) = decompress_p2(&input[end + 1..]);

        (idx_opt.map(|idx| idx + end), marker[0] * marker[1] + (length - marker[0]))
    } else {
        match input.find('(') {
            Some(n) => (Some(n), n),
            None => (None, input.len()), 
        }
    }
}

pub fn get_solution_1() -> usize {
    decompress_all(parse(include_str!("../data/d09.txt").to_string())).len()
}

pub fn get_solution_2() -> usize {
    let mut input = include_str!("../data/d09.txt").to_string();
    loop {
        let entries = parse(input);
        input = decompress_all(entries);
        if is_decompressed(&input) {
            return input.len();
        }
    }
}

#[test]
fn test_entry_parse() {
    let input = "A(1x5)BC";
    let (entry, idx) = Entry::parse(input.to_string());
    assert_eq!(idx, Some(1));
    assert_eq!(entry.marker, [0, 0]);
    assert_eq!(entry.chars, "A");
    
    let (entry, idx) = Entry::parse(input[idx.unwrap()..].to_string());
    assert!(idx.is_none());
    assert_eq!(entry.marker, [1, 5]);
    assert_eq!(entry.chars, "BC");

    let (entry, idx) = Entry::parse("ADVENT".to_string());
    assert!(idx.is_none());
    assert_eq!(entry.marker, [0, 0]);
    assert_eq!(entry.chars, "ADVENT");
}

#[test]
fn test_decompress() {
    let entries = parse("ADVENT".to_string());
    assert_eq!("ADVENT", decompress_all(entries));
    
    let entries = parse("A(1x5)BC".to_string());
    assert_eq!("ABBBBBC", decompress_all(entries));
    
    let entries = parse("(3x3)XYZ".to_string());
    assert_eq!("XYZXYZXYZ", decompress_all(entries));
    
    let entries = parse("A(2x2)BCD(2x2)EFG".to_string());
    assert_eq!("ABCBCDEFEFG", decompress_all(entries));
    
    let entries = parse("(6x1)(1x3)A".to_string());
    assert_eq!("(1x3)A", decompress_all(entries));
    
    let entries = parse("X(8x2)(3x3)ABCY".to_string());
    assert_eq!("X(3x3)ABC(3x3)ABCY", decompress_all(entries));

    let entries = parse("ADVENTA(1x5)BC(3x3)XYZA(2x2)BCD(2x2)EFG(6x1)(1x3)AX(8x2)(3x3)ABCY".to_string());
    assert_eq!("ADVENTABBBBBCXYZXYZXYZABCBCDEFEFG(1x3)AX(3x3)ABC(3x3)ABCY", decompress_all(entries));
}

#[test]
fn test_p2() {
    let mut input = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".to_string();
    loop {
        let entries = parse(input);
        input = decompress_all(entries);
        if is_decompressed(&input) {
            break assert_eq!(input.len(), 445);
        }
    }
}

#[test]
fn test_decompress_p2() {
    let (idx, len) = decompress_p2("(3x3)XYZ");

    assert!(idx.is_none());
    assert_eq!(len, 9);
}

#[test]
fn test_decompress_p2_2() {
    let input = "X(8x2)(3x3)ABCY";
    let mut idx = 0;
    let mut total_len = 0;
    let total_len = loop {
        let (new_idx, len) = decompress_p2(&input[idx..]);
        total_len += len;
        match new_idx {
            Some(new_idx) => idx += new_idx,
            None => break total_len,
        }
    };
    assert_eq!(total_len, 20);
}