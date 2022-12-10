struct Cpu {
    x: i32,
    cycle: i32,
    signals_strengths: Vec<i32>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            x: 1,
            cycle: 1,
            signals_strengths: Vec::new(),
        }
    }

    fn measure(&mut self) {
        if self.cycle == 20 || (self.cycle - 20) % 40 == 0 {
            self.signals_strengths.push(self.x * self.cycle)
        }
    }

    pub fn noop(&mut self) {
        self.measure();
        self.cycle += 1;
    }

    pub fn addx(&mut self, arg: i32) {
        for _ in 0..2 {
            self.measure();
            self.cycle += 1;
        }
        self.x += arg;
    }
}

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

    let sum: i32 = cpu.signals_strengths.iter().sum();
    println!("{}", sum);
}
