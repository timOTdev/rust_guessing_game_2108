use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    // Generate a random number.
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    println!("Guess the number!");
    println!("Secret number: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // Capture the input.
        // String is of the owned type and always UTF-8.
        // new is an associated function.
        let mut guess = String::new();

        // Read the input and saves as string to variable guess.
        // Bring in libary to read the line and handle Ok or Error.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Need to convert guess number into a number.
        // Tell users to re-enter guess until type of number.
        // Shadowing also occurs here for guess variable,
        // allows the change of variable type.
        let guess: u32 = match guess
            .trim()
            .parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

        // Print the input.
        println!("You guessed: {}", guess);

        // Compare the secret number and guess.
        // Breaks the loop if you guess correctly.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too low.".red()),
            Ordering::Equal => {
                println!("{}", "You win.".green());
                break;
            },
            Ordering::Greater => println!("{}", "Too high.".red()),
        };
    }
}
