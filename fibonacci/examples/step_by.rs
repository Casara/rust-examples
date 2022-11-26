use num_format::{Locale, ToFormattedString};

use fibonacci::Fibonacci;

const STEP_SIZE: usize = 5;

fn main() {
    100u16
        .fib_iter()
        .step_by(STEP_SIZE)
        .enumerate()
        .for_each(|(n, number)| {
            println!(
                "{:3} => {}",
                n * STEP_SIZE,
                number.to_formatted_string(&Locale::en)
            )
        });
}
