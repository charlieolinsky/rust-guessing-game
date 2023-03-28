use std::{io, cmp::Ordering}; 
use rand::Rng;  

fn main() {

    //Generate Random Number using External Crate 
    let secret_number = rand::thread_rng().gen_range(1..=100); //inclusive [x, y]
    println!("The secret number is: {secret_number}");

    //Prompt User for Input
    println!("Guess the Number!"); 

    //infinte loop allows user to continue guessing 
    loop {
        println!("Please input your guess. ");

        //Retrieve and Store User Input
        let mut guess = String::new(); 
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to Read Line"); 

        //we must change the type of guess to be an integer so that we can compare it to secret_number
        //parse returns a Result enum (Ok or Err) that we can use to handle erroneous inputs with match
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        }; 

        //Read back
        println!("You guessed {}", guess); 
        
        //Compare guess to secret number and prompt accordingly
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!!!");
                break; //BREAK LOOP WHEN GUESS IS CORRECT 
            }
        }
    }
    
    
}
