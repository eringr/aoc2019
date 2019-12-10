
const MEM_LEN: usize = 165;
static STARTING_MEM: [i32; MEM_LEN] = [
1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,9,19,23,1,13,23,27,1,5,27,31,2,31,6,35,1,35,5,39,1,9,39,43,1,43,5,47,1,47,5,51,2,10,51,55,1,5,55,59,1,59,5,63,2,63,9,67,1,67,5,71,2,9,71,75,1,75,5,79,1,10,79,83,1,83,10,87,1,10,87,91,1,6,91,95,2,95,6,99,2,99,9,103,1,103,6,107,1,13,107,111,1,13,111,115,2,115,9,119,1,119,6,123,2,9,123,127,1,127,5,131,1,131,5,135,1,135,5,139,2,10,139,143,2,143,10,147,1,147,5,151,1,151,2,155,1,155,13,0,99,2,14,0,0
];

const ADD:i32 = 1;
const MUL:i32 = 2;

fn cycle(mem: &mut [i32; MEM_LEN], pc: usize) -> usize {
    let inst = mem[pc];
    let p1 = mem[mem[pc+1] as usize];
    let p2 = mem[mem[pc+2] as usize];
    match inst {
        ADD => {
            mem[mem[pc+3] as usize] = p1 + p2;
            return 4;
        },
        MUL => {
            mem[mem[pc+3] as usize] = p1 * p2;
            return 4;
        },
        _ => return 0,
    }
}

fn attempt_sentence(noun: i32, verb: i32) -> i32 {
    let mut mem = STARTING_MEM.clone();
    mem[1] = noun;
    mem[2] = verb;
    let mut advance = 4;
    let mut pc = 0;
    while advance != 0 {
        advance = cycle(&mut mem, pc);
        pc += advance;
    }
    return mem[0];
}

fn main() {
    let upper = 85;
    'outer: for i in 0..upper {
        for j in 0..upper {
            if attempt_sentence(i, j) == 19690720 {
                println!("Correct result found with noun {} and verb {}", i, j);
                break 'outer;
            }
        }
    }
}
