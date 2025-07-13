use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let contents = get_file(&parse_args());
    let (line_count, word_count, char_count) = get_count(&contents);
    

    println!("Line Count: {}", line_count);
    println!("Word Count: {}", word_count);
    println!("Character Count: {}", char_count);


    if word_count <= 100 {
        println!("What is this, a haiku? Come on, you can do better than that ya lazy butt");
        return;
    }

    if word_count <= 250 {
        println!("It's a start I guess. Shakespeare must be quivering in his grave that the bard of this era is soooooo far along...");
        return;
    }

    if word_count <= 500 {
        println!("Mmm, that's getting nice and wordy. I especially like the part where you mixed up 'your' and 'you're' so keep it up.");
        return;
    }

    if word_count <= 1000 {
        println!("Now that's what I'm talking about. Now you have 1/10 of a lunatic's political manifesto or 10 terrible Sonic slash fics");
        return;
    }

    if word_count > 1000 {
        println!("V-vegeta, what does the scouter say? This semi competent writer's power level is over 1000?! NO WAY! We'd better flex and loudly grunt for the next 6 hours!");
        return;
    }
}


fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: cargo run <file_path>");
    }

    args[1].clone()
}

fn get_file(path: &str) -> String {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("Error opening file: {} EVERYTHING GO EXPLOD!", err)
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        panic!("Error reading file: {} I HAVE BEEN DISHONORED! *DIES*", err);
    }

    contents
}

fn get_count(text: &String) -> (usize, usize, usize) {
    let line_count = text.lines().count();
    let word_count = text.split_whitespace().count();
    let char_count = text.chars().count();
    (line_count, word_count, char_count)
}