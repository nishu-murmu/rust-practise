extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
// first ever game in rust => cli guessing game
fn main() {
    loop {
        let secret_number: i32 = rand::thread_rng().gen_range(1, 10);
        let mut guess = String::new();
        println!("Guess the number =>");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("you guessed equal number");
                break;
            }
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too Less!"),
        }
    }
}
