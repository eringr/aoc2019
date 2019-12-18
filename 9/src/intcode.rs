
use std::vec::Vec;

pub struct IntcodeComputer {
    mem: Vec::<i64>,
    relative_base: i64,
    inputs: std::collections::VecDeque::<i64>,
    outputs: std::collections::VecDeque::<i64>,
    pc: RunMode,
}

impl IntcodeComputer {
    pub fn new(mem_in: &[i64]) -> IntcodeComputer {
        let mut mem = Vec::<i64>::new();
        mem.extend_from_slice(mem_in);
        mem.resize(2000, 0);
        IntcodeComputer {
            mem: mem,
            relative_base: 0,
            inputs: std::collections::VecDeque::new(),
            outputs: std::collections::VecDeque::new(),
            pc: RunMode::Running(0),
        }
    }

    pub fn push_input(&mut self, x: i64) {
        self.inputs.push_back(x);
    }

    pub fn pop_output(&mut self) -> Option<i64> {
        self.outputs.pop_front()
    }

    pub fn cycle(&mut self) -> RunMode {
        if let RunMode::WaitingForInput(x) = self.pc {
            self.pc = RunMode::Running(x);
        }
        loop {
            let pc = match self.pc {
                RunMode::Halted => return self.pc,
                RunMode::WaitingForInput(_) => return self.pc,
                RunMode::Running(x) => x,
            };
            
            let mem = &mut self.mem;
            let inst = mem[pc];
            let (opcode, mode_p1, mode_p2, mode_p3) = (
                inst % 100,
                (inst / 100) % 10,
                (inst / 1000) % 10,
                (inst / 10000) % 10,
            );
            let relative_base = self.relative_base;

            let get_param = |param:i64, encoding:i64| -> i64 {
                match encoding {
                    0 => mem[param as usize],
                    1 => param,
                    2 => mem[(relative_base + param) as usize],
                    _ => panic!("Invalid param encoding"),
                }
            };
            let get_output_addr = |param:i64, encoding:i64| -> usize {
                match encoding {
                    0 => param as usize,
                    2 => (param + relative_base) as usize,
                    _ => panic!("Invalid output encoding"),
                }
            };
            let p1 = || get_param(mem[pc+1], mode_p1);
            let p2 = || get_param(mem[pc+2], mode_p2);

            match Mnemonics::from_i64(opcode) {
                Mnemonics::Add => {
                    let output_addr = get_output_addr(mem[pc+3], mode_p3);
                    mem[output_addr] = p1() + p2();
                    self.pc = RunMode::Running(pc+4);
                },
                Mnemonics::Mul => {
                    let output_addr = get_output_addr(mem[pc+3], mode_p3);
                    mem[output_addr] = p1() * p2();
                    self.pc = RunMode::Running(pc+4);
                },
                Mnemonics::In => {
                    if let Some(x) = self.inputs.pop_front() {
                        println!("Input: {}", x);
                        let output_addr = get_output_addr(mem[pc+1], mode_p1);
                        mem[output_addr as usize] = x;
                        self.pc = RunMode::Running(pc+2);
                    } else {
                        self.pc = RunMode::WaitingForInput(pc);
                    }
                },
                Mnemonics::Out => {
                    println!("Output: {}", p1());
                    self.outputs.push_back(p1());
                    self.pc = RunMode::Running(pc+2);
                },
                Mnemonics::Bne => {
                    self.pc = if p1() != 0i64 {RunMode::Running(p2() as usize)} else
                        {RunMode::Running(pc+3)};
                },
                Mnemonics::Beq => {
                    self.pc = if p1() == 0i64 {RunMode::Running(p2() as usize)} else
                        {RunMode::Running(pc+3)};
                },
                Mnemonics::Slt => {
                    let output_addr = get_output_addr(mem[pc+3], mode_p3);
                    mem[output_addr] = if p1() < p2() {1} else {0};
                    self.pc = RunMode::Running(pc+4);
                },
                Mnemonics::Seq => {
                    let output_addr = get_output_addr(mem[pc+3], mode_p3);
                    mem[output_addr] = if p1() == p2() {1} else {0};
                    self.pc = RunMode::Running(pc+4);
                },
                Mnemonics::Arb => {
                    let p1 = p1();
                    self.relative_base += p1;
                    self.pc = RunMode::Running(pc+2);
                },
                _ => {
                    println!("Halting.");
                    self.pc = RunMode::Halted;
                },
            }
        }
    }
}

enum Mnemonics {Add,Mul,In,Out,Bne,Beq,Slt,Seq,Und,Arb}

impl Mnemonics {
    fn from_i64(o: i64) -> Self {
        match o {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::In,
            4 => Self::Out,
            5 => Self::Bne,
            6 => Self::Beq,
            7 => Self::Slt,
            8 => Self::Seq,
            9 => Self::Arb,
            _ => Self::Und,
        }
    }
}

#[derive(Copy, Clone)]
pub enum RunMode {
    Halted,
    Running(usize),
    WaitingForInput(usize),
}
