use std::io;
use rand::Rng;


fn main() {
    println!("guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("generated secret number{secret_number}");



    print!("Please input your guess: ");

    let mut guess: String = String::new();
    
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

}