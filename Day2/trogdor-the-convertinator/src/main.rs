use std::{io};

fn main() {
    // Prompt the user for the type of conversion they want to perform
    println!("Welcome to Trogdor the Convertinator! Convertinating all the peoples.");
    println!("Please select what kind of unit you want to convert");
    
    // Display the conversion options
    let choice: u8 = prompt_user_int("1: Temperature\n2: Distance");
    match choice {
        1 => {
            convert_temperature();
            return;
        }
        2 => {
            convert_distance();
            return;
        }
        _ => {
            println!("Invalid choice, please try again.");
            return;
        }
    };
}

fn convert_temperature() {
    // Prompt the user for the type of temperature conversion they want to perform
    println!("Which temperature unit do you want to convert?");
    let choice: u8 = prompt_user_int("1: Celsius to Fahrenheit\n2: Fahrenheit to Celsius");
    
    // Convert Celsius to Fahrenheit
    match choice {
        1 => {
            let celsius: f64 = prompt_user_float("Enter the temperature in Celsius: ".to_string());
            let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
            println!("{} Celsius is {} Fahrenheit", celsius, fahrenheit);
        }
        2 => {
            // Convert Fahrenheit to Celsius
            let fahrenheit: f64 = prompt_user_float("Enter the temperature in Fahrenheit: ".to_string());
            let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
            println!("{} Fahrenheit is {} Celsius", fahrenheit, celsius);
        }
        _ => {
            println!("Invalid choice, please try again.");
            convert_temperature();
        }
    }
}


fn convert_distance() {
    // Prompt the user for the type of distance conversion they want to perform
    println!("Which distance unit do you want to convert?");
    let choice: u8 = prompt_user_int("1: Kilometers to Miles\n2: Miles to Kilometers");
    
    match choice {
        1 => {
            // Convert Kilometers to Miles
            let kilometers: f64 = prompt_user_float("Enter the distance in Kilometers: ".to_string());
            let miles = kilometers * 0.621371; //621, the unit no furry will have trouble remembering
            println!("{} Kilometers is {} Miles", kilometers, miles);
        }
        2 => {
            // Convert Miles to Kilometers
            let miles: f64 = prompt_user_float("Enter the distance in Miles: ".to_string());
            let kilometers = miles / 0.621371;
            println!("{} Miles is {} Kilometers", miles, kilometers);
        }
        _ => {
            println!("Invalid choice, please try again.");
            convert_distance();
        }
    }
}

// Function to prompt the user for an integer input and return it
// This function will keep prompting until a valid integer is entered
fn prompt_user_int(prompt: &str) -> u8 {
    loop {
        println!("{}", prompt);
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        
        match choice.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("That isn't one of the choices kiddo"),
        }
    }
}

// Function to prompt the user for a float input and return it
// This function will keep prompting until a valid float is entered
fn prompt_user_float(prompt: String) -> f64 {
    loop {
        println!("{}", prompt);
        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("That isn't one of the choices kiddo");
        
        match choice.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("That isn't one of the choices kiddo"),
        }
    }
}