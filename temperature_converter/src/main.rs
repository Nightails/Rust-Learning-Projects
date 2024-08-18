use std::io;

fn main() {
    println!("Temperature Converter");

    println!("Select a converter:");
    println!("1 - Fahrenheit to Celsius");
    println!("2 - Celsius to Fahrenheit");
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line.");
    let option: u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if option == 1 {
        println!("Please input your temperature (Fahrenheit):");
    } else if option == 2 {
        println!("Please input your temperature (Celsius):");
    }
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line.");

    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    if option == 1 {
        let result = (temperature - 32.0) * (5.0/9.0);
        println!("Result: {}C", result);
    } else if option == 2 {
        let result = (9.0/5.0) * temperature + 32.0;
        println!("Result: {}F", result);
    }
}
