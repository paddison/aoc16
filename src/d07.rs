fn parse(input: &'static str) -> Vec<IP7> {
    input.lines().map(|line| line.into()).collect()
}

struct IP7 {
    hypernet_seq: Vec<&'static str>,
    seq: Vec<&'static str>,
}

impl IP7 {
    fn supports_tls(&self) -> bool {
        for part in &self.hypernet_seq {
            if Self::verify_part_tls(part) {
                return false;
            }
        }

        for part in &self.seq {
            if Self::verify_part_tls(part) {
                return true;
            }
        }

        false
    }

    fn verify_part_tls(part: &str) -> bool {
        for (((a, b), c), d) in part.chars().zip(part[1..].chars()).zip(part[2..].chars()).zip(part[3..].chars()) {
            if a == d && b == c && a != b {
                return true;
            }
        }

        false
    }

    fn supports_ssl(&self) -> bool {
        for part in &self.seq {
            for chars in Self::verify_part_ssl(part) {
                for hyper_part in &self.hypernet_seq {
                    if hyper_part.contains(&chars) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn verify_part_ssl(part: &str) -> Vec<String> {
        let mut patterns = Vec::new();

        for ((a, b), c) in part.chars().zip(part[1..].chars()).zip(part[2..].chars()) {
            if a == c && a != b {
                patterns.push(format!("{}{}{}", b, a, b));
            }
        }
        patterns
    }
}

impl From<&'static str> for IP7 {
    fn from(input: &'static str) -> Self {
        let mut hypernet_seq = Vec::new();
        let mut seq = Vec::new();

        let mut seq_start = 0;
        for ((h_start, _), (h_end, _)) in input.match_indices('[').zip(input.match_indices(']')) {
            seq.push(&input[seq_start..h_start]);
            hypernet_seq.push(&input[h_start + 1..h_end]);
            seq_start = h_end + 1;
        }

        seq.push(&input[seq_start..]);

        Self { hypernet_seq, seq }
    }
}

pub fn get_solution_1() -> usize {
    parse(include_str!("../data/d07.txt")).iter().filter(|ip| ip.supports_tls()).count()
}

pub fn get_solution_2() -> usize {
    parse(include_str!("../data/d07.txt")).iter().filter(|ip| ip.supports_ssl()).count()
}

#[test]
fn test_ip7_from_str() {
    let ip = IP7::from("abba[mnop]qrst[ab]cdef");
    assert_eq!(ip.hypernet_seq.len(), 2);
    assert_eq!(ip.hypernet_seq[0], "mnop");
    assert_eq!(ip.hypernet_seq[1], "ab");
    assert_eq!(ip.seq.len(), 3);
    assert_eq!(ip.seq[0], "abba");
    assert_eq!(ip.seq[1], "qrst");
    assert_eq!(ip.seq[2], "cdef");
}

#[test]
fn test_verify_part() {
    assert!(IP7::verify_part_tls( "abba"));
    assert!(!IP7::verify_part_tls( "abbx"));
    assert!(IP7::verify_part_tls( "ioxxoj"));
    assert!(!IP7::verify_part_tls( "aaaa"));
}