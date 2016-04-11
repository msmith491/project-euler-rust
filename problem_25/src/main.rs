extern crate num;

use num::bigint::{ToBigInt, BigInt};

fn main() {

    struct Fib {
        curr: BigInt,
        next: BigInt
    }

    impl Fib {
        fn new() -> Fib {
            Fib {curr: 0.to_bigint().unwrap(),
                 next: 0.to_bigint().unwrap()}
        }
    }

    impl Iterator for Fib {
        type Item = BigInt;

        fn next(&mut self) -> Option<BigInt> {
            if  self.next == 0.to_bigint().unwrap() {
                self.next = 1.to_bigint().unwrap();
                return Some(1.to_bigint().unwrap());
            }
            let tmp = self.curr.clone();
            self.curr = self.next.clone();
            self.next = self.curr.clone() + tmp;
            Some(self.next.clone())
        }
    }

    let mut f = Fib::new();

    for (index, elem) in f.enumerate() {
        let curr = elem.to_string();
        if curr.len() >= 1000 {
            println!("{}", index + 1);
            break;
        }
    }
}
