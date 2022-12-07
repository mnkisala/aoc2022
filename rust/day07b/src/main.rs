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

    const DISK_SPACE: u32 = 70000000;
    const TARGET: u32 = DISK_SPACE - 30000000;

    let total_used: u32 = dirs[&vec!["/".into()]];
    println!("total used: {}", total_used);

    let best_candidate: u32 = dirs
        .values()
        .map(|v| *v)
        .filter(|size| (total_used - size) <= TARGET)
        .min()
        .unwrap();

    println!("{}", best_candidate);
}
