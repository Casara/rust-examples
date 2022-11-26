use num_format::{Locale, ToFormattedString};

use fibonacci::Fibonacci;

fn main() {
    100u8
        .fib_iter()
        .enumerate()
        .for_each(|(n, number)| println!("{:3} => {}", n, number.to_formatted_string(&Locale::en)));
}
