// Constant for freezing point of water in Fahrenheit
const FREEZING_POINT_F: f64 = 32.0;

// Convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    let mut temp_f: f64 = FREEZING_POINT_F;

    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{}°F = {:.2}°C", temp_f, temp_c);

    // Convert back to Fahrenheit using the second function
    let back_to_f = celsius_to_fahrenheit(temp_c);
    println!("{:.2}°C = {:.2}°F", temp_c, back_to_f);

    for _ in 1..=5 {
        temp_f += 1.0;
        let converted = fahrenheit_to_celsius(temp_f);
        println!("{}°F = {:.2}°C", temp_f, converted);
    }
}
