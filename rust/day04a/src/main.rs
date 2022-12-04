use std::ops::Range;

fn parse_range(input: &str) -> Range<u32> {
    let range: Vec<&str> = input.split('-').collect();
    Range {
        start: range[0].parse().unwrap(),
        end: range[1].parse().unwrap(),
    }
}

fn parse_pair(input: &str) -> (Range<u32>, Range<u32>) {
    let pair: Vec<&str> = input.split(',').collect();
    (parse_range(pair[0]), parse_range(pair[1]))
}

fn is_fully_included(r: &Range<u32>, src: &Range<u32>) -> bool {
    r.start >= src.start && r.end <= src.end
}

fn main() {
    let count = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|line| {
            let (a, b) = parse_pair(line);
            is_fully_included(&a, &b) || is_fully_included(&b, &a)
        })
        .count();
    println!("{}", count);
}
