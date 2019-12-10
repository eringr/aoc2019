
fn test_val(input: i32) -> bool {
    let get_digit = |input: i32, place: u32| -> i32 {
        (input / 10i32.pow(place)) % 10
    };
    let mut highest = 0i32;
    let mut counts = [0; 10];
    for n in (0..6).rev() {
        let digit = get_digit(input, n);
        if digit < highest {
            return false;
        }
        highest = digit;
        counts[digit as usize] += 1;
    }
    if counts.iter().any(|&n| n == 2) {
        return true;
    }
    false
}

fn main() {
    let mut total = 0u32;
    for n in 240920..=789857 {
        if test_val(n) {
            total += 1;
        }
    }
    println!("Found {} in range that meet criteria.", total);
}