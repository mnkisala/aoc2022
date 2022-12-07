fn is_unique(slice: &[char]) -> bool {
    for (ai, a) in slice.iter().enumerate() {
        for (bi, b) in slice.iter().enumerate() {
            if ai == bi {
                continue;
            } else if a == b {
                return false;
            }
        }
    }

    true
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let chars: Vec<char> = input.chars().collect();

    for i in 4..chars.len() {
        if is_unique(&chars[(i - 4)..i]) {
            println!("{:?}", i);
            break;
        }
    }
}
