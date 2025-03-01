fn main() {
    // Pulse > 100 = tachycardia, < 60 = bradycardia—ICU 101. See Rust by Example “Control Flow” for `if` magic!
    let pulse = 120; // Swap this with your last patient’s HR
    let spo2 = 97;  // Tweak this for SpO2 fun (normal: 95-100%)
    let bp_sys = 145;  // Systolic BP, tweak for fun (normal: 90-140)
    let bp_dia = 95;   // Diastolic BP, tweak for fun (normal: 60-90)
    let rr = 22;      // Respiratory rate, tweak for fun (normal for adult: 12-20)

    let map = (2 * bp_dia + bp_sys) / 3;  // Calculate MAP—ICU realness!

    let mut alerts = Vec::new();  // Define alerts here—Rust needs it!

    if pulse > 100 {
        alerts.push(format!("Tachycardia alert! HR: {}", pulse));
    } else if pulse < 60 {
        alerts.push(format!("Bradycardia alert! HR: {}", pulse));
    } else {
        alerts.push(format!("Pulse chill. HR: {}", pulse));
    }

    if spo2 < 90 {
        alerts.push(format!("Hypoxia alert! SpO2: {}", spo2));
    } else if spo2 > 100 {
        alerts.push(format!("SpO2 must be < 100! You enter SpO2 = {}. Check input!", spo2));
    } else {
        alerts.push(format!("SpO2 normal: {}", spo2));
    }

    if bp_sys > 140 && bp_dia > 90 {
        alerts.push(format!("Hypertension alert! BP: {} / {}", bp_sys, bp_dia));
    } else if bp_sys < 90 && bp_dia < 60 {
        alerts.push(format!("Hypotension alert! BP: {} / {}", bp_sys, bp_dia));
    } else if bp_sys >= 180 || bp_dia >= 120 {
        alerts.push(format!("Hypertension crisis alert! BP: {} / {}—Call for help NOW!", bp_sys, bp_dia));
    } else {
        alerts.push(format!("BP chill. BP: {} / {}", bp_sys, bp_dia));
    }

    if map < 60 {
        alerts.push(format!("Low MAP alert! MAP: {}—Risk of hypoperfusion!", map));
    } else if map > 100 {
        alerts.push(format!("High MAP alert! MAP: {}—Hypertension risk!", map));
    } else {
        alerts.push(format!("MAP normal: {}", map));
    }

    if rr > 20 {
        alerts.push(format!("Tachypnea alert! RR: {}", rr));
    } else if rr < 12 {
        alerts.push(format!("Bradypnea alert! RR: {}", rr));
    } else {
        alerts.push(format!("RR chill. RR: {}", rr));
    }

    println!("\nVital Status:");
    for alert in alerts {
        println!("{}", alert);
    }
}