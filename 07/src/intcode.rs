
use std::vec::Vec;

pub struct IntcodeComputer {
    mem: Vec::<i32>,
    inputs: std::collections::VecDeque::<i32>,
    outputs: std::collections::VecDeque::<i32>,
    pc: RunMode,
}

impl IntcodeComputer {
    pub fn new(mem_in: &[i32]) -> IntcodeComputer {
        let mut mem = Vec::<i32>::new();
        mem.extend_from_slice(mem_in);
        IntcodeComputer {
            mem: mem,
            inputs: std::collections::VecDeque::new(),
            outputs: std::collections::VecDeque::new(),
            pc: RunMode::Running(0),
        }
    }

    pub fn push_input(&mut self, x: i32) {
        self.inputs.push_back(x);
    }

    pub fn pop_output(&mut self) -> Option<i32> {
        self.outputs.pop_front()
    }

    pub fn cycle(&mut self) -> RunMode {
        if let RunMode::WaitingForInput(x) = self.pc {
            self.pc = RunMode::Running(x);
        }
        while let RunMode::Running(pc) = self.pc {
            let mut mem = &mut self.mem;
            let inst = mem[pc];
            let (opcode, mode_p1, mode_p2, mode_p3) = (
                inst % 100,
                (inst / 100) % 10,
                (inst / 1000) % 10,
                (inst / 10000) % 10,
            );

            let get_param = |param:i32, encoding:i32| -> i32 {
                if encoding == 0 {mem[param as usize]} else {param}
            };
            let p1 = || get_param(mem[pc+1], mode_p1);
            let p2 = || get_param(mem[pc+2], mode_p2);
            let p3 = || get_param(mem[pc+3], mode_p3);

            match Mnemonics::from_i32(opcode) {
                Mnemonics::Add => {
                    let output_addr = mem[pc+3] as usize;
                    mem[output_addr] = p1() + p2();
                    self.pc = RunMode::Running(pc+4);
                },
                Mnemonics::Mul => {
                    let output_addr = mem[pc+3] as usize;
                    mem[output_addr] = p1() * p2();
                    self.pc = RunMode::Running(pc+4);
                },
                Mnemonics::In => {
                    if let Some(x) = self.inputs.pop_front() {
                        println!("Input: {}", x);
                        let output_addr = mem[pc+1];
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
                    self.pc = if p1() != 0i32 {RunMode::Running(p2() as usize)} else
                        {RunMode::Running(pc+3)};
                },
                Mnemonics::Beq => {
                    self.pc = if p1() == 0i32 {RunMode::Running(p2() as usize)} else
                        {RunMode::Running(pc+3)};
                },
                Mnemonics::Slt => {
                    let output_addr = mem[pc+3] as usize;
                    mem[output_addr] = if p1() < p2() {1} else {0};
                    self.pc = RunMode::Running(pc+4);
                },
                Mnemonics::Seq => {
                    let output_addr = mem[pc+3] as usize;
                    mem[output_addr] = if p1() == p2() {1} else {0};
                    self.pc = RunMode::Running(pc+4);
                }
                _ => {
                    println!("Halting.");
                    self.pc = RunMode::Halted;
                },
            }
        }
        self.pc
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
pub enum RunMode {
    Halted,
    Running(usize),
    WaitingForInput(usize),
}
