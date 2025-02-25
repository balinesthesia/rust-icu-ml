fn main() {
    // Pulse > 100? That’s tachycardia—ICU 101. See Rust by Example “Control Flow” for the `if` magic!
    let pulse = 120; // Change this value with yours to play with the program
    let spo2 = 97; // Tweak this for SpO2 fun (normal: 95-100%)
    if pulse > 100 {
        println!("Tachycardia alert! HR: {}", pulse);
    } else if pulse < 60 {
        println!("Bradycardia alert! HR: {}", pulse);
    } else {
        println!("Pulse chill. HR: {}", pulse);
    }

    if spo2 < 95 {
        println!("Hypoxia alert! SpO2: {}", spo2);
    } else if spo2 > 100 {
        println!(
            "SpO2 must be < 100! You enter SpO2 = {}. Please check your input again.",
            spo2
        );
    } else {
        println!("SpO2 normal: {}", spo2);
    }
}
