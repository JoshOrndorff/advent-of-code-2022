use Instruction::*;
use sscanf::sscanf;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CPU {
    x: i32,
    clock: u32,
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

    let p: Vec<_> = program.clone().collect();
    println!("program {:?}", p);

    let mut signal_strengths = Vec::<i32>::new();
    let mut cpu = CPU::default();

    for inst in program {

        let next_state = cpu.next_state(inst);

        let mut next_target_cycle = 20 + signal_strengths.len() as u32 * 40;
        while next_state.clock >= next_target_cycle {
            println!("next target cycle: {:?}", next_target_cycle);
            signal_strengths.push(cpu.x * <u32 as TryInto<i32>>::try_into(next_target_cycle).unwrap());
            next_target_cycle = 20 + signal_strengths.len() as u32 * 40;
        }


        println!("cpu: {:?}", cpu);
        println!("signal strengths: {:?}\n\n", signal_strengths);

        cpu = next_state;
    }


    // Saving this line in case I need it later
    // Takes an iterator of instructions and runs it to completion
    //.fold(CPU::default(), |prev_state, inst| prev_state.next_state(inst));

    

    println!("Hello, world!: {:?}", signal_strengths.iter().sum::<i32>());
}
