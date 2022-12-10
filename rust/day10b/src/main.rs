fn main() {
    let mut cpu = Cpu::new();

    for line in std::io::stdin().lines().map(Result::unwrap) {
        let mut tok = line.split(' ');

        match tok.next().unwrap() {
            "noop" => {
                cpu.noop();
            }
            "addx" => {
                let val = tok.next().unwrap().parse::<i32>().unwrap();
                cpu.addx(val);
            }
            _ => panic!("unknnown instruction"),
        }
    }

    cpu.crt.print();
}

struct Cpu {
    x: i32,
    cycle: i32,
    crt: Crt,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            x: 1,
            cycle: 1,
            crt: Crt {
                display: ['.'; 240],
            },
        }
    }

    fn draw(&mut self) {
        self.crt.draw(self.cycle - 1, self.x - 1);
    }

    pub fn noop(&mut self) {
        self.draw();
        self.cycle += 1;
    }

    pub fn addx(&mut self, arg: i32) {
        for _ in 0..2 {
            self.draw();
            self.cycle += 1;
        }
        self.x += arg;
    }
}

struct Crt {
    display: [char; 240],
}

impl Crt {
    fn draw(&mut self, offset: i32, pos: i32) {
        self.display[offset as usize % 240] = if (pos..pos + 3).contains(&(offset % 40)) {
            '#'
        } else {
            '.'
        };
    }

    pub fn print(&self) {
        for y in 0..6 {
            for x in 0..40 {
                print!("{}", self.display[(y * 40) + x])
            }
            print!("\n");
        }
    }
}
