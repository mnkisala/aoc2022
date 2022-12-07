use std::collections::HashMap;

fn main() {
    let mut current_path: Vec<String> = vec![];

    let mut dirs: HashMap<Vec<String>, u32> = HashMap::new();

    for line in std::io::stdin().lines().map(Result::unwrap) {
        if line.starts_with('$') {
            let tokens: Vec<&str> = line.split(' ').collect();
            if tokens[1] == "cd" {
                match tokens[2] {
                    ".." => {
                        current_path.pop();
                    }
                    "/" => current_path = vec!["/".to_owned()],
                    _ => current_path.push(tokens[2].to_owned()),
                };
            }
        } else if line.starts_with("dir") {
        } else {
            let tokens: Vec<&str> = line.split(' ').collect();
            let size = tokens[0].parse::<u32>().unwrap();

            let mut p = current_path.clone();
            while !p.is_empty() {
                dirs.entry(p.clone())
                    .and_modify(|e| *e += size)
                    .or_insert(size);
                p.pop();
            }
        }
    }

    let sum: u32 = dirs.values().filter(|size| **size <= 100000).sum();

    println!("{}", sum);
}
