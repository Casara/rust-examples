use rust_decimal::{prelude::ToPrimitive, MathematicalOps};
use rust_decimal_macros::dec;

pub fn binet(n: u64) -> u128 {
    dec!(5)
        .sqrt()
        .map(|sqrt5| {
            let phi = (dec!(1) + sqrt5) / dec!(2);
            phi.powd(n.into()) / sqrt5
        })
        .and_then(|d| d.round().to_u128())
        .unwrap()
}

pub fn tail_recursive(n: u64) -> u128 {
    fn tail_iter(nth: u64, prev: u128, curr: u128) -> u128 {
        match nth {
            0 => prev,
            _ => tail_iter(nth - 1, curr, prev + curr),
        }
    }
    tail_iter(n, 0, 1)
}

pub fn iterative(n: u64) -> u128 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => {
            let mut curr = 1;
            let mut prev = 1;
            for _i in 3..=n {
                std::mem::swap(&mut curr, &mut prev);
                curr += prev;
            }
            curr
        }
    }
}

// Complexity: exponential
pub fn recursive(n: u64) -> u128 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => recursive(n - 1) + recursive(n - 2),
    }
}

#[derive(Clone)]
pub struct FibonacciIterator {
    last_nth: u64,
    current: u64,
}

impl Iterator for FibonacciIterator {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.le(&self.last_nth) {
            false => None,
            true => {
                let number = self.current.nth_fib();
                self.current += 1;
                Some(number)
            }
        }
    }
}

pub trait Fibonacci {
    fn fib_iter(&self) -> FibonacciIterator {
        self.fib_iter_start_at(0)
    }

    fn fib_iter_start_at(&self, nth: u64) -> FibonacciIterator;

    fn nth_fib(&self) -> u128;
}

macro_rules! fib_impl {
    ($SelfT:ty) => {
        impl Fibonacci for $SelfT {
            fn fib_iter_start_at(&self, nth: u64) -> FibonacciIterator {
                FibonacciIterator {
                    last_nth: self.to_u64().and_then(|n| n.checked_add(nth)).unwrap(),
                    current: nth,
                }
            }

            fn nth_fib(&self) -> u128 {
                match self.lt(&3) || self.gt(&138) {
                    false => binet(self.to_u64().unwrap()),
                    true => tail_recursive(self.to_u64().unwrap()),
                }
            }
        }
    };
}

fib_impl!(usize);
fib_impl!(u8);
fib_impl!(u16);
fib_impl!(u32);
fib_impl!(u64);
fib_impl!(u128);

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    const FIRST_TEN: [u128; 10] = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    const FROM_70TH_TO_80TH: [u128; 11] = [
        190392490709135,
        308061521170129,
        498454011879264,
        806515533049393,
        1304969544928657,
        2111485077978050,
        3416454622906707,
        5527939700884757,
        8944394323791464,
        14472334024676221,
        23416728348467685,
    ];

    #[test]
    fn test_binet() {
        FIRST_TEN.iter().enumerate().for_each(|(n, expected)| {
            let calculated = binet(n as u64 + 1);
            assert_eq!(*expected, calculated)
        });
    }

    #[test]
    fn test_binet_from_70th_to_80th() {
        FROM_70TH_TO_80TH
            .iter()
            .enumerate()
            .for_each(|(n, expected)| assert_eq!(*expected, binet(n as u64 + 70)));
    }

    #[test]
    fn test_tail_recursive() {
        FIRST_TEN
            .iter()
            .enumerate()
            .for_each(|(n, expected)| assert_eq!(*expected, tail_recursive(n as u64 + 1)));
    }

    #[test]
    fn test_iterative() {
        FIRST_TEN
            .iter()
            .enumerate()
            .for_each(|(n, expected)| assert_eq!(*expected, iterative(n as u64 + 1)));
    }

    #[test]
    fn test_recursive() {
        FIRST_TEN
            .iter()
            .enumerate()
            .for_each(|(n, expected)| assert_eq!(*expected, recursive(n as u64 + 1)));
    }

    #[test]
    fn test_fibonacci_trait() {
        assert_eq!(
            FROM_70TH_TO_80TH.to_vec(),
            10u64.fib_iter_start_at(70).take(11).collect::<Vec<_>>(),
        );
        assert_eq!(30_960_598_847_965_113_057_878_492_448, 138u128.nth_fib());
        assert_eq!(9_969_216_677_189_303_386_214_405_760_200, 150u128.nth_fib());
    }
}
