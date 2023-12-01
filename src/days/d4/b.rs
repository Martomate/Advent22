struct Section {
    from: i32,
    to: i32,
}

impl Section {
    fn overlaps(&self, other: &Section) -> bool {
        other.to >= self.from && other.from <= self.to
    }
}

fn parse_section(s: &str) -> Option<Section> {
    match s.split_once('-') {
        Some((l, r)) => match (l.parse::<i32>(), r.parse::<i32>()) {
            (Ok(from), Ok(to)) => Some(Section { from, to }),
            _ => None,
        },
        _ => None,
    }
}

fn parse_sections(s: &str) -> Option<(Section, Section)> {
    match s.split_once(',') {
        Some((l, r)) => match (parse_section(l), parse_section(r)) {
            (Some(ls), Some(rs)) => Some((ls, rs)),
            _ => None,
        },
        _ => None,
    }
}

pub fn main(input: &str) -> usize {
    println!("Hello, world!");

    let c = input
        .lines()
        .take_while(|l| !l.is_empty())
        .filter_map(parse_sections)
        .filter(|(l, r)| l.overlaps(r))
        .count();

    c
}
