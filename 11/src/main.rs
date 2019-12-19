
mod intcode;
use std::fmt::Debug;

const DIM: usize = 150;
const MEM_LEN: usize = 664;
type MemType = [i64; MEM_LEN];
static STARTING_MEM: MemType = [
3,8,1005,8,342,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1002,8,1,29,2,1006,19,10,1,1005,19,10,2,1102,11,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,1001,8,0,62,2,1009,15,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,1002,8,1,88,2,1101,6,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,102,1,8,114,1,105,8,10,1,1102,18,10,2,6,5,10,1,2,15,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,153,1,105,15,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,102,1,8,178,1,1006,15,10,1006,0,96,1006,0,35,1,104,7,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,102,1,8,214,1006,0,44,2,1105,17,10,1,1107,19,10,1,4,16,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,252,1006,0,6,1,1001,20,10,1006,0,45,2,1109,5,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,102,1,8,287,2,101,20,10,2,1006,18,10,1,1009,9,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,321,101,1,9,9,1007,9,1031,10,1005,10,15,99,109,664,104,0,104,1,21102,48210117528,1,1,21102,1,359,0,1105,1,463,21102,932700763028,1,1,21102,370,1,0,1105,1,463,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,1,179557207079,1,21102,417,1,0,1105,1,463,21102,1,28994202816,1,21101,0,428,0,1105,1,463,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,709580710756,1,21102,1,451,0,1106,0,463,21102,825016201984,1,1,21101,462,0,0,1106,0,463,99,109,2,21201,-1,0,1,21102,40,1,2,21101,0,494,3,21102,1,484,0,1105,1,527,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,489,490,505,4,0,1001,489,1,489,108,4,489,10,1006,10,521,1101,0,0,489,109,-2,2105,1,0,0,109,4,1201,-1,0,526,1207,-3,0,10,1006,10,544,21102,1,0,-3,21202,-3,1,1,22102,1,-2,2,21102,1,1,3,21102,563,1,0,1105,1,568,109,-4,2106,0,0,109,5,1207,-3,1,10,1006,10,591,2207,-4,-2,10,1006,10,591,21202,-4,1,-4,1105,1,659,22102,1,-4,1,21201,-3,-1,2,21202,-2,2,3,21102,610,1,0,1106,0,568,21201,1,0,-4,21102,1,1,-1,2207,-4,-2,10,1006,10,629,21102,1,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,651,21202,-1,1,1,21102,1,651,0,106,0,526,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2106,0,0
];

fn main() {
    let mut grid_raw = vec![Color::Unpainted; DIM * DIM];
    let mut grid_base: Vec<_> =
        grid_raw.as_mut_slice().chunks_mut(DIM).collect();
    let grid: &mut [&mut [_]] = grid_base.as_mut_slice();

    let mut r = Robot {
        x: DIM/2,
        y: DIM/2,
        dir: Direction::Up,         
        com: intcode::IntcodeComputer::new(&STARTING_MEM[..]),
    };

    grid[DIM/2][DIM/2] = Color::White;
    while cycle_robot(&mut r, grid) {}

    let mut total = 0;
    for n in grid {
        for (i, m) in n.iter().enumerate() {
            if let Color::Unpainted = *m {} else {total += 1}
            print!("{:?}", m);
        }
        println!("");
    }
    println!("Total painted at least once: {}", total);
}

fn cycle_robot(robot: &mut Robot, grid: &mut [&mut [Color]]) -> bool {
    let color_input = match grid[robot.y][robot.x] {
        Color::Unpainted | Color::Black => 0,
        Color::White => 1,
    };
    robot.com.push_input(color_input);
    let runmode = robot.com.cycle();
    let new_color = match robot.com.pop_output().expect("") {
        0 => Color::Black,
        1 => Color::White,
        _ => panic!("Unexpected color")
    };
    println!("{:?}", new_color);
    robot.dir = match robot.com.pop_output().expect("") {
        0 => Direction::left_from(robot.dir),
        1 => Direction::right_from(robot.dir),
        _ => panic!("Unexpected direction"),
    };
    let (new_x, new_y) = match robot.dir {
        Direction::Up    => (robot.x, robot.y - 1),
        Direction::Down  => (robot.x, robot.y + 1),
        Direction::Left  => (robot.x - 1, robot.y),
        Direction::Right => (robot.x + 1, robot.y),
    };
    // println!("painting {} {} to {:?}", robot.x, robot.y, new_color);
    grid[robot.y][robot.x] = new_color;
    robot.x = new_x;
    robot.y = new_y;
    if let intcode::RunMode::Halted = runmode {false} else {true}
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left_from(d: Direction) -> Direction {
        match d {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn right_from(d: Direction) -> Direction {
        Direction::left_from(
            Direction::left_from(
                Direction::left_from(d)
            )
        )
    }
}

#[derive(Clone, Copy)]
enum Color {
    Unpainted,
    Black,
    White,
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Color::Unpainted => write!(f, "."),
            Color::Black     => write!(f, "."),
            Color::White     => write!(f, "#"),
        }
    }
}

struct Robot {
    x: usize,
    y: usize,
    dir: Direction,
    com: intcode::IntcodeComputer,
}
