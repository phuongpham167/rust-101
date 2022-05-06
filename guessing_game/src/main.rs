use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello!");
    let today = "Thursday"; //immutable
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    for n in 1..5 {
        let mut guess = String::new(); //mut - mutable
        println!("Please input your guess turn {}: ", n);
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess :u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        }; //shadow the previous value of variable
        println!("You guessed in {}: {}", today, guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
                if n == 4 {
                    println!("=>>>>>Looser");
                }
            },
            Ordering::Greater => {
                println!("Too big");
                if n == 4 {
                    println!("=>>>>>Looser");
                }
            },
            Ordering::Equal => {
                println!("You win!!!!!!");
                break;
            }
        }
    }
    
}
