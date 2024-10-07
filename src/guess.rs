//Imports
use rand::Rng;
use std::{cmp::Ordering, io, thread, time};

//code
pub fn guessing_number_game() {
    println!("Lets begin guessing number game");
    thread::sleep(time::Duration::from_millis(700));
    pub fn guess() -> u16 {
        let mut guess = String::new();
        println!("please input your guessed number");
        io::stdin()
            .read_line(&mut guess)
            .expect("Input is not a number");
        let guess: u16 = guess.trim().parse().expect("AAAGGGGGHHHH!");
        guess
    }

    pub fn guess_rand() -> u16 {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        secret_number
    }

    pub fn comp(guess: u16, secret_number: u16) {
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => println!("You guessed it right!"),
        }
    }
    // Main game logic
    let secret_number = guess_rand(); // Generate the secret number
                                      //  println!("Secret number = {secret_number}"); checking loop is exiting or not
    loop {
        let guess = guess(); // Get the user's guess
        comp(guess, secret_number); // Compare the guess with the secret number

        // Break the loop if the guess is correct
        if guess == secret_number {
            break; // Exit the loop if the guess is correct
        }
    }
}
