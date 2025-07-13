use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("🌋 Welcome to Guessr! The world's only game show filmed over a live volcano! 🌋");
    println!("I'm thinking of a number between 1 and 100. You have 10 guesses before a trapdoor opens beneath you, sealing your fate >:3");

    // Generate the target number
    let mut guesses = 1;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // Use a little light threatening to get a guess
        println!("Guess Number {}", guesses);

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input. To the lava with you all!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number. You must like plummeting to your doom >:3");
                continue;
            }
        };

        // Use ordering crate to check results cause it's fancier than if statments 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Uh oh, that's too smol 🤏"),
            Ordering::Greater => println!("Whoops, that's too big 🍆"),
            Ordering::Equal => {
                println!("🎊 Correct! It only took you {} tries to avoid a very unpleasant but admirably flashy demise! 🎊", guesses);
                break;
            }
        }

        guesses += 1;
        if guesses == 10 {
            println!("Alas, that was your last try. Thanks for playing, we'll see you next fall! *Pushes trapdoor button* 😈👇 🔥🔥 💀👻");
        }
    }
}
