pub mod cli {
    use std::io::{self, Write};
    use std::{thread, time::Duration};
    use colored::*;

    pub fn welcome_banner() {
        println!("{}", "╔══════════════════════════════════════════════════════════════╗".bright_magenta());
        println!("{}", "║                                                              ║".bright_magenta());
        println!("{}", "║    🦊  ✨  STOOPID FOX TASK MANAGER  ✨  🦊                  ║".bright_yellow().bold());
        println!("{}", "║                                                              ║".bright_magenta());
        println!("{}", "║           /\\_/\\    Let me help you organize                  ║".bright_cyan());
        println!("{}", "║          < ^w^ >   your tasks, fren! UwU                     ║".bright_cyan());
        println!("{}", "║           > ^ <                                              ║".bright_cyan());
        println!("{}", "║                                                              ║".bright_magenta());
        println!("{}", "╚══════════════════════════════════════════════════════════════╝".bright_magenta());
        println!();
    }

    pub fn main_menu() {
        println!("{}", "┌─────────────────────────────────────┐".bright_blue());
        println!("{}", "│           🌟 MAIN MENU 🌟           │".bright_blue());
        println!("{}", "├─────────────────────────────────────┤".bright_blue());
        println!("{} {}", "│ 1.".bright_white(), "Add a task                   UwU │".green());
        println!("{} {}", "│ 2.".bright_white(), "Add important task           8w8 │".yellow());
        println!("{} {}", "│ 3.".bright_white(), "View tasks                   OwO │".cyan());
        println!("{} {}", "│ 4.".bright_white(), "Mark task completed          ^w^ │".bright_green());
        println!("{} {}", "│ 5.".bright_white(), "Delete a task                XwX │".red());
        println!("{} {}", "│ 6.".bright_white(), "Exit                         ;w; │".bright_black());
        println!("{}", "└─────────────────────────────────────┘".bright_blue());
        println!();
    }

    pub fn cute_typing_animation(text: &str, color: Color) {
        for char in text.chars() {
            print!("{}", char.to_string().color(color));
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(50));
        }
        println!();
    }

    pub fn loading_animation(message: &str) {
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        print!("{} ", message.bright_cyan());
        io::stdout().flush().unwrap();
        
        for _ in 0..20 {
            for frame in &frames {
                print!("\r{} {}", message.bright_cyan(), frame.bright_magenta());
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(20));
            }
        }
        println!("\r{} {}", message.bright_cyan(), "✨".bright_yellow());
    }

    pub fn get_input(prompt: &str) -> String {
        print!("{} {}", "-w-".bright_yellow(), prompt.bright_white());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
    
    pub fn get_input_as_int(prompt: &str) -> u8 {
        loop {
            print!("{} {}", "-w-".bright_yellow(), prompt.bright_white());
            io::stdout().flush().unwrap();
    
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
    
            match input.trim().parse::<u8>() {
                Ok(num) => return num,
                Err(_) => {
                    error_message("Das not a number between 1 and 6!!!! (╯°□°）╯");
                }
            }
        }
    }

    pub fn success_message(message: &str) {
        println!("{} {}", "✨".bright_green(), message.bright_green().bold());
    }

    pub fn error_message(message: &str) {
        println!("{} {}", "💥".red(), message.bright_red().bold());
    }

    pub fn info_message(message: &str) {
        println!("{} {}", "ℹ️".bright_blue(), message.bright_blue());
    }
}