use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    // we do not need to mute the secret number variable here.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");
        // Variables are inmutable by default mut changes that.
        // We want the user to keep trying to guess our number, mut makes that possible. 
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Shawdowing is being used on the guess variable here.
        // Trim takes a number from 5\n to 5 eliminating whitespace beg. to end to stage a comparision of two diff types of data
        // Parse method parses the string into some kind of number, the : allows us to annotate the variable's type. "Hey I want this to be a number!"
        // Rust infers that secret number is a u32 and now the comparison is of the same type!!
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {} address: {:p}", guess, &guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
            }
        }
    }

}
