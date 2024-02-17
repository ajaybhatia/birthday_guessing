use std::io;

use crate::{TABLE_1, TABLE_2, TABLE_3, TABLE_4, TABLE_5};

pub fn get_input(index: usize) -> i32 {
    let table = match index {
        1 => TABLE_1,
        2 => TABLE_2,
        3 => TABLE_3,
        4 => TABLE_4,
        5 => TABLE_5,
        _ => "",
    };
    println!("{}", table);
    println!("Do you see your birthday? (y/n)");

    loop {
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line!");
        match answer.trim() {
            "y" | "n" => return if answer.trim() == "y" { 1 } else { 0 },
            _ => println!("Please enter 'y' or 'n'"),
        }
    }
}
