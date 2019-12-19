
fn test_val(input: i32) -> bool {
    let get_digit = |place: u32| -> i32 {
        (input / 10i32.pow(place)) % 10
    };
    let mut highest = 0i32;
    let mut counts = [0; 10];
    for n in (0..6).rev() {
        let digit = get_digit(n);
        if digit < highest {
            return false;
        }
        highest = digit;
        counts[digit as usize] += 1;
    }
    counts.iter().any(|&n| n == 2)
}

fn main() {
    let total = (240920..=789857).filter(|&n| test_val(n)).count();
    println!("Found {} in range that meet criteria.", total);
}
