extern crate num;

use num::bigint::{ToBigInt,BigInt};
use num::pow;

fn main() {

    fn digit_sum(num: BigInt) -> usize {
        num
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .fold(0, |a, b| a + b) as usize
    }

    fn sum_pow_digits(num: BigInt, p: usize) -> usize {
        let pnum = pow(num, p);
        digit_sum(pnum)
    }

    let num = (1..100)
        .map(|x| (1..100)
             .map(|y| sum_pow_digits(x.to_bigint().unwrap(), y))
             .max()
             .unwrap())
        .max()
        .unwrap();

    println!("{:?}", num)
}
