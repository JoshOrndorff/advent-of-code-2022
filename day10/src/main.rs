use Instruction::*;
use sscanf::sscanf;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CPU {
    x: i32,
    clock: i32,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            x: 1,
            clock: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        if let Ok(()) =  sscanf!(s, "noop") {
            return Noop;
        }
        if let Ok(x) = sscanf!(s, "addx {}", i32) {
            return Addx(x);
        }

        panic!("Couldn't parse instruction: {}", s)
    }
}

impl CPU {
    /// Calculates and returns the next machine state.
    fn next_state(&self, inst: Instruction) -> Self {
        match inst {
            Noop => Self { x: self.x, clock: self.clock + 1 },
            Addx(x) => Self{ x: self.x + x, clock: self.clock + 2 },
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("should read input file");
    let program = input.trim().lines().map(Instruction::from);
    
    // ---------- Part 1 ---------------------

    let mut signal_strengths = Vec::<i32>::new();
    let mut cpu = CPU::default();

    for inst in program.clone() {

        let next_state = cpu.next_state(inst);

        let mut next_target_cycle = 20 + signal_strengths.len() as i32 * 40;
        while next_state.clock >= next_target_cycle {
            signal_strengths.push(cpu.x * next_target_cycle);
            next_target_cycle = 20 + signal_strengths.len() as i32 * 40;
        }

        cpu = next_state;
    }

    println!("signal strength: {:?}", signal_strengths.iter().sum::<i32>());

    // ---------- Part 2 ---------------------
    let mut program = program;

    let mut cpu = CPU::default();
    let mut cpu_next = cpu.next_state(program.next().expect("should have at least one instruction"));

    'outer:
    for row in 0..6 {
        for col in 0..40 {
            let current_cycle = row * 40 + col + 1;

            if current_cycle > cpu_next.clock {
                cpu = cpu_next;
                let inst = match program.next() {
                    Some(inst) => inst,
                    None => {
                        break 'outer;
                    }
                };
                cpu_next = cpu_next.next_state(inst);
            }

            if i32::abs(col - cpu.x) <= 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
