extern crate num;

use num::num_bigint::{ToBigInt, BigInt};
use num::pow;

fn main() {

    fn series_sum(n: usize) -> BigInt {

        (1..(n+1))
            .map(|x| pow(x.to_bigint().unwrap(), x))
            .fold(0.to_bigint().unwrap(), |a, i| a + i)
    }

    println!("{:?}", &series_sum(1000)
             .to_string()
             .chars()
             .rev()
             .collect::<String>()[0..10]
             .chars()
             .rev()
             .collect::<String>()
             .parse::<usize>()
             .unwrap());
}
