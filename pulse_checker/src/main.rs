fn main() {
    // Pulse > 100? That’s tachycardia—ICU 101. See Rust by Example “Control Flow” for the `if` magic!
    let pulse = 120; // Change this value with yours to play with the program
    if pulse > 100 {
        println!("Tachiycardia alert! HR: {}", pulse);
    } else if pulse < 60 {
        println!("Bradycardia alert! HR: {}", pulse);
    } else {
        println!("Pulse chill. HR: {}", pulse);
    }
}
