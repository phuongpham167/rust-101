use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello!");
    let mut guess = String::new(); //mut - mutable
    let today = "Thursday"; //immutable
    println!("Please input your guess: ");
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess :u32 = guess.trim().parse().expect("Please type a number"); //shadow the previous value of variable
    println!("You guessed in {}: {}", today, guess);
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win")
    }
}
