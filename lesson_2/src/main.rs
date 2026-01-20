//Programing a guessing game
use std::io;
use std::cmp::Ordering;

use rand::Rng;


fn main() {

    println!("guess the number");

    println!("Input");

    let num = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed!!!!");
    

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            },
        };
    
        println!("Your Guess --> {guess}");

        match guess.cmp(&num) {
            Ordering::Equal => {
                println!("You win");
                break;
            },
            Ordering::Greater => println!("Big!!!!!!"),
            Ordering::Less => println!("Small!!!!!")
        }

    }

}
