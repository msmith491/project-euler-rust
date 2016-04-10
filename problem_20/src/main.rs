extern crate num;

use num::bigint::ToBigInt;

fn main() {

    // fn factorial(n: usize) -> usize {
    //     (1..(n + 1)).fold(1, |acc, item| acc * item)
    // }

    let bignum = (1..101)
        .fold(1.to_bigint().unwrap(),
              |acc, item|
              acc * item.to_bigint().unwrap())
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
        .iter()
        .fold(0, |acc, item| acc + item);

    println!("{:?}", bignum);
}
