use num_format::{Locale, ToFormattedString};

use fibonacci::Fibonacci;

const NTH: usize = 100;

fn main() {
    let mut it = NTH.fib_iter().cycle();

    println!(
        "{:3} => {}",
        NTH,
        it.nth(NTH).unwrap().to_formatted_string(&Locale::en)
    );

    for n in 0..=5 {
        println!(
            "{:3} => {}",
            n,
            it.next().unwrap().to_formatted_string(&Locale::en)
        )
    }
}
