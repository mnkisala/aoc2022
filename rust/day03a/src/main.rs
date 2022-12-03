fn priority(c: char) -> u32 {
    let v: u32 = c.into();
    if v >= 'a'.into() {
        return v - u32::from('a') + 1;
    } else {
        return v - u32::from('A') + 1 + 26;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_works() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('c'), 3);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('C'), 29);
        assert_eq!(priority('Z'), 52);
    }
}

fn main() {
    let sum: u32 = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);

            a.chars()
                .filter_map(|ac| if b.contains(ac) { Some(ac) } else { None })
                .nth(0)
                .unwrap()
        })
        .map(priority)
        .sum();

    println!("{}", sum);
}
