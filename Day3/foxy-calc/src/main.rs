use std::{char};
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

fn main() {
    let welcome_message = "Welcome to Foxy Calc. Just type in math expressions and watch it update live. Press 'q' to quit and press 'p' to pet me!";
    let invalid_input_message = "Sorry chief, that's not a valid formula.";
    prompt(welcome_message, invalid_input_message);
}

fn prompt(message: &str, invalid_input_message: &str) -> String {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    print!("{}{}{}\r\n", clear::All, cursor::Goto(1, 1), message);
    stdout.flush().unwrap();

    let (mut operand1, mut result, mut operator) = (String::new(), 0.0, char::default());
    let mut has_result = false;
    
    for c in stdin.keys() {
        match c.as_ref().unwrap() {
            Key::Char('p') => {
                print!("\r\n^w^ Purrrr~\r\n");
                stdout.flush().unwrap();
            },
            Key::Char('q') => {
                print!("\r\nExiting...\r\n");
                stdout.flush().unwrap();
                break;
            },
            Key::Char('+') | Key::Char('-') | Key::Char('*') | Key::Char('/') | Key::Char('%') => {
                if let Key::Char(op) = c.unwrap() {
                    if !operand1.is_empty() {
                        if has_result && operator != char::default() {
                            // Chain operations: apply previous operator first
                            if let Ok(current_operand) = operand1.parse::<f64>() {
                                result = calculate(result, current_operand, operator);
                                print!("\r\nResult: {}\r\n", result);
                                stdout.flush().unwrap();
                            }
                        } else if !has_result {
                            // First operation: set result to operand1
                            result = operand1.parse::<f64>().unwrap_or(0.0);
                            has_result = true;
                        }
                        
                        operator = op;
                        operand1.clear();
                        print!("{}", op);
                        stdout.flush().unwrap();
                    }
                }
            },
            Key::Char('=') => {
                if !operand1.is_empty() && operator != char::default() && has_result {
                    if let Ok(current_operand) = operand1.parse::<f64>() {
                        result = calculate(result, current_operand, operator);
                        print!("\r\nResult: {}\r\n", result);
                        
                        // Reset for next calculation
                        operand1 = result.to_string();
                        operator = char::default();
                        has_result = true;
                        stdout.flush().unwrap();
                    }
                } else {
                    print!("\r\n{}\r\n", invalid_input_message);
                    stdout.flush().unwrap();
                }
            },
            Key::Char(digit) if digit.is_digit(10) || *digit == '.' => {
                operand1.push(*digit);
                print!("{}", digit);
                stdout.flush().unwrap();
            },
            _ => {}
        }
    }
    "temp".to_string()
}

fn calculate(a: f64, b: f64, operator: char) -> f64 {
    match operator {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => if b != 0.0 { a / b } else { panic!("SYsTeM FAiLurE! DIVIDING BY ZERO AAAAAAAA! XwX"); }, // Handle division by zero
        '%' => a % b,
        _ => a,
    }
}