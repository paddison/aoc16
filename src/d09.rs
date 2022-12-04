fn parse_marker(input: &str) -> (usize, usize, usize) {
    match (input.find('('), input.find(')')) {
        (Some(0), Some(end)) => {
            let parts = &input[1..end].split('x').collect::<Vec<&str>>();
            (parts[0].parse::<usize>().unwrap(), parts[1].parse::<usize>().unwrap(), end)
        },
        _ => panic!("invalid slice")
    }
}

fn find_next_marker(input: &str) -> Option<usize> {
    input.find('(')
}

fn get_decompressed_length(input: &str) -> usize {
    let mut cursor = 0;
    let mut length = 0;
    while let Some(marker_idx) = find_next_marker(&input[cursor..]) {
        if marker_idx > 0 {
            length += marker_idx; // contains no marker 
            cursor += marker_idx;
        }
        let (n_chars, repeats, marker_end) = parse_marker(&input[cursor..]);
        length += n_chars * repeats;
        cursor += marker_end + 1 + n_chars;
    }
    
    length += &input[cursor..].len();

    length
}

fn decompress_all(input: &str) -> usize {
    if find_next_marker(input).is_none() {
        return input.len();
    }
    let mut cursor = 0;
    let mut length = 0;
    while let Some(m_idx) = find_next_marker(&input[cursor..]) {
        if m_idx > 0 {
            length += m_idx;
            cursor += m_idx;
        }
        let (n_chars, repeats, m_end) = parse_marker(&input[cursor..]);
        cursor += m_end + 1;
        length += repeats * decompress_all(&input[cursor..cursor + n_chars]);
        cursor +=  n_chars;
    }

    return length + input.len() - cursor
}

pub fn get_solution_1() -> usize {
    get_decompressed_length(include_str!("../data/d09.txt"))
}

pub fn get_solution_2() -> usize {
    decompress_all(include_str!("../data/d09.txt"))
}

#[test]
fn test_get_decompressed_length() {
    assert_eq!(get_decompressed_length("A(2x2)BCD(2x2)EFG"), 11);
}

#[test]
fn test_decompress_all() {
    assert_eq!(decompress_all("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
}