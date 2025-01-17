/*
  [Chapter]
  chapter="2"
  [dependencies]
  console = "0.15.10"
  rand = "0.8.5"
*/
use std::io;
use rand::Rng;
use std::cmp::Ordering;

use console::style; // you can remove this dependency I just like to have it

fn main() {
  loop {
    let mut guess: String = String::new();
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);

    // read the input into the mutable variable called "guess"
    io::stdin()
      .read_line(&mut guess)
      .expect("Invalid string / input");
    
    /* 
      - parse guess to a u32 so that we can use it when comparing the 2 values
      - using match to handle the error instead of crashing on error (aka invalid input)
    */
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num, // if it succeeded it will return the number 
      Err(_) => continue, // continue just tells the program to go to the next iteration of the loop 
    };

    println!("You guessed: {guess}");

    match guess.cmp(&random_number) {
      Ordering::Less => println!("{}", style("Too small!").red()),
      Ordering::Greater => println!("{}", style("Too big!").red()),
      Ordering::Equal => {
          println!("{}", style("You win!").green());
          break; // break out of the lo op
      },
    }

    println!("The number is {random_number}");
  } 
}
