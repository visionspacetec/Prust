use std::io;
use rand;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    let target = rand::random::<i8>() % 101;

    //println!("Random : {}",target);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    //let guess
    while guess.trim().parse::<i8>().expect("Conversion error") != target
    {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
    }

}