fn main() {
    // Patient's Vital Signs Data

    let pulse = 120; // Change this value with yours to play with the program
    let spo2 = 97; // Tweak this for SpO2 fun (normal: 95-100%)
    let bp_sys = 145; // Systolic BP, tweak for fun (normal: 90-140)
    let bp_dia = 95; // Diastolic BP, tweak for fun (normal: 60-90)

    // Heartrate (HR) Check
    if pulse > 100 {
        println!("Tachycardia alert! HR: {}", pulse);
    } else if pulse < 60 {
        println!("Bradycardia alert! HR: {}", pulse);
    } else {
        println!("Pulse chill. HR: {}", pulse);
    }

    // SpO2 Check
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

    // Blood Pressure (BP) Check
    if bp_sys > 140 && bp_dia > 90 {
        println!("Hypertension alert! BP: {} / {}", bp_sys, bp_dia);
    } else if bp_sys < 90 && bp_dia < 60 {
        println!("Hypotension alert! BP: {} / {}", bp_sys, bp_dia);
    } else if bp_sys >= 180 || bp_dia >= 120 {
        println!(
            "Hypertension crisis alert! BP: {} / {}—Call for help NOW!",
            bp_sys, bp_dia
        );
    } else {
        println!("BP chill. BP: {} / {}", bp_sys, bp_dia);
    }
}
