use util::constants::*;
use util::helper::get_input;

mod util;

fn main() {
    println!("Welcome to the birthday guessing game!");
    println!("Answer the following five questions with either 'y' or 'n'.");
    println!("We will then guess the day of the month you were born on.");
    println!("*********************************************************");

    let mut answers = vec![0; 5];

    for i in 0..5 {
        answers[i] = get_input(i + 1);
    }

    let result = answers
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &x)| acc + x * (1 << i));

    println!("\nYour birthday is on the {}th day of the month!", result);
}
