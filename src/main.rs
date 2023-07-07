use std::io; // io library to work with data input/output

fn main() {
    println!("Guess the number!");

    println!("Please, type your guess");

    let mut user_guess = String::new();
    
    io::stdin() 
        .read_line(&mut user_guess) // here we mutate variable user_guess and write user prompt in user_guess. & - indicates that user_guess is refference
        .expect("Failed to read line"); // fallback (??)

    println!("You guessed: {user_guess}");

    sandbox()
}

// testing print line feature
fn sandbox() {
    let x = 5;
    let y = 10;

    println!("x = {x} and y = {}", y)
}