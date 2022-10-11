use std::io;
use clap::Parser;

#[derive(Parser)]
struct Game {
    guess_word: String,
}
fn main() {
    let args = Game::parse();
    let word: Vec<&str> = args.guess_word.split("").filter(|s| !s.is_empty()).collect();
    let mut input = String::new(); 
    let mut i = 8;
    let mut empty: Vec<String> = vec![];
    let mut guessed: Vec<String> = vec![];
    let mut output: &str;
    for _n in word.iter() {
        empty.push("_".to_string());
    }
    //clearing terminal and initial prompt
    print!("\x1B[2J\x1B[1;1H");
    for n in empty.iter() {
        print!("{}", n)
    }
    //main game loop
     while i > 0 {
        //check if won
        match empty.contains(&"_".to_string()) {
            true =>
            { 
                println!("\n\nEnter a letter:");
                io::stdin().read_line(&mut input)
                    .ok()
                    .expect("Failed to read line");
                // ä/ö/ü is longer than 1
                if input.contains("ä") || input.contains("ö") || input.contains("ü") {
                    input.truncate(2);
                }
                else {
                    input.truncate(1);
                }
                
                print!("\x1B[2J\x1B[1;1H");
                //check if letter already guessed
                match guessed.contains(&input.trim().to_string()) {
                    true =>
                    {
                        output = "You already guessed that letter!";
                    }
                    false =>
                    {
                        //change empty(_) vector
                        if word.contains(&input.trim()) && input.trim() != "" {
                            for (i, item) in word.iter().enumerate() {
                                if item == &input.trim() {
                                    empty[i] = item.to_string();
                                }
                            }
                            output = "That's right!";
                        }
                        else {
                            output = "Wrong :(";
                            i = i - 1;
                        }
                        guessed.push(input.trim().to_string());
                    } 
                }
                for n in empty.iter() {
                    print!("{}", n)
                }
                println!("\n\n{output}");
                println!("Tries left: {}\n", i);
                print!("Guessed letters: ");
                for letter in guessed.iter() {
                    print!("{}, ", letter);
                }
                if i == 0 {
                    println!("\n\nYou Lost. :(")
                }
                input.clear();
            }
            
            false => 
            {
                println!("\n\nYou win!!!!! :)");
                break;
            }
        }
    }
}

