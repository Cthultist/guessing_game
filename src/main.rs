//packages we need for this program
use std::io; //standard input/output package
use std::cmp::Ordering; //compare ordering
use rand::Rng; //random number generator

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); //generates random number between range, in this case 1-100.

    loop { //loop that grabs user unput, parses it into a number or continues, and compares against the secret_number. 
        println!("Please input your guess.");

        let mut guess = String::new(); //saves user input as an empty mutable string in variable "guess".

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line."); //receives input data from the user and has .expect as error handling.

        let guess: u32 = match guess.trim().parse() { //saves user input in guess variable, uses a match statement as error handling.
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { //the match arms to compare the user input against the random generated number.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;
            }
        }
    }
}
