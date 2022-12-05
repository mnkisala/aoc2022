fn main() {
    let mut init = true;
    let mut first_line = true;
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for line in std::io::stdin().lines().map(Result::unwrap) {
        if first_line {
            let n_stacks = (line.len() + 1) / 4;
            stacks = vec![Vec::new(); n_stacks];

            let mut i = 0;
            for c in line.chars() {
                if i % 4 == 1 {
                    if c.is_alphabetic() {
                        stacks[i / 4].push(c);
                    }
                }
                i += 1;
            }

            first_line = false;
        } else if init {
            if line.len() == 0 {
                for stack in &mut stacks {
                    stack.reverse();
                }
                init = false;
            } else {
                // this code is duplicated, oh well
                let mut i = 0;
                for c in line.chars() {
                    if i % 4 == 1 {
                        if c.is_alphabetic() {
                            stacks[i / 4].push(c);
                        }
                    }
                    i += 1;
                }
            }
        } else {
            let (amount, from, to) = {
                // parsing (kind of)
                let mut iter = line.split(' ');
                let _ = iter.next().unwrap();
                let amount = iter.next().unwrap().parse::<usize>().unwrap();
                let _ = iter.next().unwrap();
                let from = iter.next().unwrap().parse::<usize>().unwrap();
                let _ = iter.next().unwrap();
                let to = iter.next().unwrap().parse::<usize>().unwrap();

                (amount, from, to)
            };

            for _ in 0..amount {
                let el = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(el);
            }
        }
    }

    let res: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    println!("{}", res);
}
