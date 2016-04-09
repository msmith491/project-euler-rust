extern crate num;

use num::bigint::ToBigInt;

fn main() {

    let mut bignum = 2.to_bigint().unwrap();
    let bignum2 = 2.to_bigint().unwrap();
    for _ in 1..1000 {
        bignum = bignum * &bignum2;
    }

    let s = bignum
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
        .iter()
        .fold(0, |acc, item| acc + item);

    println!("{}", s);
}
