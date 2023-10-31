use std::{io::stdin, cmp::Ordering};
use rand::{thread_rng, Rng};
fn main() {
    println!("Welcome to Guessing Game");
    let secret = thread_rng().gen_range(1, 10);
    let mut round = 3;

    while round > 0 {
        println!("Enter a number B/W 0 & 10: ");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        let guessed_num: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a Valid Number!!");
                continue;
            },
        };

        match guessed_num.cmp(&secret) {
            Ordering::Equal => {
                 round = -1;
                 break;
            },
            Ordering::Less => println!("Greater Value"),
            Ordering::Greater => println!("Lesser")
        }
        round = round - 1
    }

    if round == 0 {
        println!("Your chances are up");
        println!("The Secret number is {}", secret)
    } else if round == -1 {
        println!("Your Guess is Right!!!");
    }

}
