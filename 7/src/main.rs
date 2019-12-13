use permutohedron::heap_recursive;

const N_AMPLIFIERS: usize = 5;
const MEM_LEN: usize = 515;
type MemType = [i32; MEM_LEN];

static STARTING_MEM: MemType = [
3,8,1001,8,10,8,105,1,0,0,21,38,59,84,93,110,191,272,353,434,99999,3,9,101,5,9,9,1002,9,5,9,101,5,9,9,4,9,99,3,9,1001,9,3,9,1002,9,2,9,101,4,9,9,1002,9,4,9,4,9,99,3,9,102,5,9,9,1001,9,4,9,1002,9,2,9,1001,9,5,9,102,4,9,9,4,9,99,3,9,1002,9,2,9,4,9,99,3,9,1002,9,5,9,101,4,9,9,102,2,9,9,4,9,99,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,99
];

fn main() {
    let mut data: [i32; N_AMPLIFIERS] = [5, 6, 7, 8, 9];
    let mut permutations = Vec::new();
    heap_recursive(&mut data, |permutation| {
        permutations.push(permutation.to_vec())
    });
    let mut max_seen = 0;
    for p in permutations {
        let m = attempt_permutation(&p);
        if m > max_seen {
            max_seen = m;
            println!("New max seen: {}, permutation: {:?}", max_seen, p);
        }
    }
    println!("Max seen: {}", max_seen);
}

fn attempt_permutation(p: &std::vec::Vec<i32>) -> i32 {
    println!("Attempting {:?}", p);
    let mut computers = std::vec::Vec::<IntcodeComputer>::new();
    for n in 0..N_AMPLIFIERS {
        let mut c = IntcodeComputer::new();
        c.push_input(p[n]);
        computers.push(c);
    }
    computers[0].push_input(0);
    let mut last_output = 0;
    loop {
        let mut halted = true;
        for n in 0..N_AMPLIFIERS {
            let next = if n+1 == N_AMPLIFIERS {0} else {n+1};
            let result = computers[n].cycle();
            while !computers[n].outputs.is_empty() {
                let val = computers[n].pop_output();
                if next == 0 {
                    last_output = val;
                }
                computers[next].push_input(val);
            }
            if let RunMode::WaitingForInput(_) = result {
                halted = false;
            }
        }
        if halted {
            return last_output;
        }
    }
}

struct IntcodeComputer {
    mem: MemType,
    inputs: std::collections::VecDeque::<i32>,
    outputs: std::collections::VecDeque::<i32>,
    pc: RunMode,
}

impl IntcodeComputer {
    fn new() -> IntcodeComputer {
        IntcodeComputer {
            mem: STARTING_MEM.clone(),
            inputs: std::collections::VecDeque::new(),
            outputs: std::collections::VecDeque::new(),
            pc: RunMode::Running(0),
        }
    }

    fn push_input(&mut self, x: i32) {
        self.inputs.push_back(x);
    }

    fn pop_output(&mut self) -> i32 {
        self.outputs.pop_front().expect("")
    }

    fn cycle(&mut self) -> RunMode {
        if let RunMode::WaitingForInput(x) = self.pc {
            self.pc = RunMode::Running(x);
        }
        loop {
            let pc = match self.pc {
                RunMode::Halted => return self.pc,
                RunMode::WaitingForInput(_) => return self.pc,
                RunMode::Running(x) => x,
            };
            
            let mut mem = &mut self.mem;
            let inst = mem[pc];
            let (opcode, mode_p1, mode_p2) = (
                inst % 100,
                (inst / 100) % 10,
                (inst / 1000) % 10,
            );

            let get_param = |param:i32, encoding:i32| -> i32 {
                if encoding == 0 {mem[param as usize]} else {param}
            };
            let p1 = || get_param(mem[pc+1], mode_p1);
            let p2 = || get_param(mem[pc+2], mode_p2);

            match Mnemonics::from_i32(opcode) {
                Mnemonics::Add => {
                    mem[mem[pc+3] as usize] = p1() + p2();
                    self.pc = RunMode::Running(pc+4);
                },
                Mnemonics::Mul => {
                    mem[mem[pc+3] as usize] = p1() * p2();
                    self.pc = RunMode::Running(pc+4);
                },
                Mnemonics::In => {
                    if let Some(x) = self.inputs.pop_front() {
                        println!("Input: {}", x);
                        mem[mem[pc+1] as usize] = x;
                        self.pc = RunMode::Running(pc+2);
                    } else {
                        self.pc = RunMode::WaitingForInput(pc);
                    }
                },
                Mnemonics::Out => {
                    let p1 = get_param(mem[pc+1], mode_p1);
                    println!("Output: {}", p1);
                    self.outputs.push_back(p1);
                    self.pc = RunMode::Running(pc+2);
                },
                Mnemonics::Bne => {
                    self.pc = if p1() != 0i32 {RunMode::Running(p2() as usize)} else
                        {RunMode::Running(pc+3)};
                },
                Mnemonics::Beq => {
                    self.pc = if p1() == 0i32 {RunMode::Running(p2() as usize)} else
                        {RunMode::Running(pc+3)};
                },
                Mnemonics::Slt => {
                    mem[mem[pc+3] as usize] = if p1() < p2() {1} else {0};
                    self.pc = RunMode::Running(pc+4);
                },
                Mnemonics::Seq => {
                    mem[mem[pc+3] as usize] = if p1() == p2() {1} else {0};
                    self.pc = RunMode::Running(pc+4);
                }
                _ => self.pc = RunMode::Halted,
            }
        }
    }
}

enum Mnemonics {Add,Mul,In,Out,Bne,Beq,Slt,Seq,Und}

impl Mnemonics {
    fn from_i32(o: i32) -> Self {
        match o {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::In,
            4 => Self::Out,
            5 => Self::Bne,
            6 => Self::Beq,
            7 => Self::Slt,
            8 => Self::Seq,
            _ => Self::Und,
        }
    }
}

#[derive(Copy, Clone)]
enum RunMode {
    Halted,
    Running(usize),
    WaitingForInput(usize),
}
