mod intcode;
use permutohedron::heap_recursive;
use intcode::{IntcodeComputer, RunMode};


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
        let mut c = IntcodeComputer::new(&STARTING_MEM[..]);
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
            while let Some(val) = computers[n].pop_output() {
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
