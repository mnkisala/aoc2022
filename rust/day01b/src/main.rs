fn main() {
    let mut elves: Vec<u32> = Vec::new();
    elves.push(0);

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            elves.push(0)
        } else {
            *elves.last_mut().unwrap() += line.parse::<u32>().unwrap();
        }
    }

    elves.sort();
    elves.reverse();

    let sum: u32 = elves.iter().take(3).sum();
    println!("{}", sum);
}
