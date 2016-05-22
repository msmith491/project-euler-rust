extern crate num;

use num::bigint::{ToBigInt, BigInt};

fn main() {

    fn factorial(n: BigInt) ->  BigInt {
        if n == 0.to_bigint().unwrap() {
            return 1.to_bigint().unwrap();
        }
        n.clone() * factorial(n - 1.to_bigint().unwrap())
    }

    fn ncr(n: usize, r: usize) -> BigInt {
        let facn = factorial(n.to_bigint().unwrap());
        let facr = factorial(r.to_bigint().unwrap());
        let facnr = factorial((n - r).to_bigint().unwrap());

        facn / (facr * facnr)
    }

    let mut result = Vec::new();
    for n in 2..101 {
        for r in 1..n {
            let s = ncr(n, r);
            if s >= 1000000.to_bigint().unwrap() {
                result.push(s);
            }
        }
    }

    println!("{:?}", result.len());
}
