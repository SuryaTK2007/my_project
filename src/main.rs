use std::io;
use rand::Rng;
fn main(){
    let secret=rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret}");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read your input");
    println!("Your guess is {guess}");
}