extern crate rand;
extern crate ansi_term;

use std::io;
use ansi_term::Colour::{Red, Green};

fn main() {
    loop {
        println!("Heads or Tails?");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("I didn't quite get that..");

        let guess = input.contains("h");

        if rand::random() && guess {
            println!("{}", Green.paint("You were right!"));
        } else {
            println!("{}", Red.paint("You were wrong.."));
        }
    }
}
