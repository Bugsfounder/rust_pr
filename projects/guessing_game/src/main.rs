use std::cmp::Ordering;
use std::io;
use rand::Rng;
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("Please input your guess: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        // let guess: u32 =  guess.trim().parse().expect("Please type a number");
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) =>num,
            Err(_)=> continue
        };

        // println!("Guessed number is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
