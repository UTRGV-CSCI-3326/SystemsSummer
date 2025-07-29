use std::io;

const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    println!("Enter a temperature in Fahrenheit:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let temp_f: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let temp_c = fahrenheit_to_celsius(temp_f as f64);
    println!("{}°F is {:.2}°C", temp_f, temp_c);
    for i in 1..=5 {
        let f = temp_f + i;
        let c = fahrenheit_to_celsius(f as f64);
        println!("{}°F is {:.2}°C", f, c);
    }
}