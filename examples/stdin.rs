use std::io::stdin;

use square_root_irrational::is_square_root_rational;

fn main() {
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Error while reading stdin");
        let line = line.replace('\n', "");
        if line.is_empty() {
            return;
        }
        let number: u32 = line.parse().expect("Error while parsing unsigned 32-bit integer");
        let is_rational = is_square_root_rational(number);
        match is_rational {
            true => println!("√{number} is rational"),
            false => println!("√{number} is irrational")
        };
    }
}