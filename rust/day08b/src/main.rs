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

    let max_score: usize = (1..height - 1)
        .map(|y| {
            (1..width - 1)
                .map(|x| {
                    // left
                    let mut score_left = 0;
                    for i in (0..x).rev() {
                        score_left += 1;
                        if map[y][i] >= map[y][x] {
                            break;
                        }
                    }

                    // right
                    let mut score_right = 0;
                    for i in (x + 1)..width {
                        score_right += 1;
                        if map[y][i] >= map[y][x] {
                            break;
                        }
                    }

                    // up
                    let mut score_up = 0;
                    for i in (0..y).rev() {
                        score_up += 1;
                        if map[i][x] >= map[y][x] {
                            break;
                        }
                    }

                    // down
                    let mut score_down = 0;
                    for i in (y + 1)..width {
                        score_down += 1;
                        if map[i][x] >= map[y][x] {
                            break;
                        }
                    }

                    score_left * score_right * score_up * score_down
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{}", max_score);
}
