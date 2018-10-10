use std::io;

fn main() {

    println!("If you want to calculate from celsius to fahrenheit press f, 
    if you want calculate from fahrenheit to celsius press c: (f/c)");

    let mut temperature_type = String::new();

    io::stdin().read_line(&mut temperature_type)
        .expect("Failed to read line");

    if temperature_type.trim() == "c" {
        println!("You have chosen to convert Fahrenheit to Celsius.");
        println!("Enter the temperature in Fahrenheit:");

        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = temperature.trim().parse()
            .expect("You did not put in a number");

        let celsius = fahrenheit_to_celsius(temperature);
        println!("{0} Fahrenheit is {1} Celsius", temperature, celsius)
    } else if temperature_type.trim() == "f" {
        println!("You have chosen to convert Celsius to Fahrenheit.");
        println!("Enter the temperature in Celsius:");

        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = temperature.trim().parse()
            .expect("You did not put in a number");

        let fahrenheit = celsius_to_fahrenheit(temperature);
        println!("{0} Celsius is {1} Fahrenheit", temperature, fahrenheit)
    } else {
        println!("You did not choose iether c or f");
    }
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    let c = (f - 32.0) * (5.0/9.0);
    c
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    let f = (c * (9.0/5.0)) + 32.0;
    f
}