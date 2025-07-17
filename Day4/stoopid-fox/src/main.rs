use crate::cli::cli::{welcome_banner, main_menu, get_input_as_int, cute_typing_animation, loading_animation};
use crate::tasks::tasks::TaskManager;
use colored::*;

mod tasks;
mod cli;

fn main() {
    welcome_banner();
    cute_typing_animation("Initializing fox brain.......... @w@", Color::Magenta);
    let mut task_manager = TaskManager::new();

    loop {
        main_menu();
        let choice = get_input_as_int("Choose an option (1-6): ");

        match choice {
            1 => task_manager.add_task(),
            2 => task_manager.add_important_task(),
            3 => task_manager.view_tasks(),
            4 => task_manager.mark_task_completed(),
            5 => task_manager.delete_task(),
            6 => {
                cute_typing_animation("Bye bye, I'll save everything for u UwU", Color::BrightCyan);
                loading_animation("Saving ur stuff...");
                task_manager.save_tasks();
                println!("{}", "🦊✨ Thanks for using Stoopid Fox! See you soon~ ✨🦊".bright_yellow().bold());
                break;
            },
            _ => println!("{} {}", "🤔".yellow(), "I can't count that high, silly! Please choose a number between 1 and 6~ (´･ω･`)".bright_yellow())
        }

        println!();
    }
}