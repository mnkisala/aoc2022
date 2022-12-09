use std::collections::HashSet;

fn main() {
    let mut hx: i32 = 0;
    let mut hy: i32 = 0;

    let mut knots = [(0, 0); 10];

    let mut visited_by_tail: HashSet<(i32, i32)> = HashSet::new();

    for line in std::io::stdin().lines().map(Result::unwrap) {
        let mut tokens = line.split(' ');
        let dir = tokens.next().unwrap();
        let steps = tokens.next().unwrap().parse::<i32>().unwrap();

        let (dx, dy) = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!(),
        };

        for _ in 0..steps {
            hx += dx;
            hy += dy;

            /*
                       for (back, front) in todo!() {

                       }

                       if (tx - hx).abs() == 2 || (ty - hy).abs() == 2 {
                           tx = hx - dx;
                           ty = hy - dy;
                           visited_by_tail.insert((tx, ty));
                       }
                       visited_by_tail.insert((tx, ty));
            */
        }
    }

    let visited = visited_by_tail.len(); // not counting the start
    println!("{}", visited);
}
