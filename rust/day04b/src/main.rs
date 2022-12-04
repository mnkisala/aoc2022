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

fn aabb(a: &Range<u32>, b: &Range<u32>) -> bool {
    (a.start >= b.start && a.start <= b.end)
        || (a.end >= b.start && a.end <= b.end)
        || (b.start >= a.start && b.start <= a.end)
        || (b.end >= a.start && b.end <= a.end)
}

fn main() {
    let count = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|line| {
            let (a, b) = parse_pair(line);
            aabb(&a, &b)
        })
        .count();
    println!("{}", count);
}
