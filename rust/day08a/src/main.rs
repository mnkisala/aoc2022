fn main() {
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in std::io::stdin().lines().map(Result::unwrap) {
        map.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }

    let width = map[0].len();
    let height = map.len();

    let inner_visible: usize = (1..height - 1)
        .map(|y| {
            (1..width - 1)
                .map(|x| {
                    let mut occluded = 0;

                    // left
                    for i in 0..x {
                        if map[y][i] >= map[y][x] {
                            occluded += 1;
                            break;
                        }
                    }

                    // right
                    for i in (x + 1)..width {
                        if map[y][i] >= map[y][x] {
                            occluded += 1;
                            break;
                        }
                    }

                    // up
                    for i in 0..y {
                        if map[i][x] >= map[y][x] {
                            occluded += 1;
                            break;
                        }
                    }

                    // down
                    for i in (y + 1)..width {
                        if map[i][x] >= map[y][x] {
                            occluded += 1;
                            break;
                        }
                    }

                    usize::from(occluded < 4)
                })
                .sum::<usize>()
        })
        .sum();

    let outer = 2 * width + 2 * (height - 2);
    let visible = inner_visible + outer;

    println!("{}", visible);
}
