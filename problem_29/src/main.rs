extern crate num;

use std::iter;
use num::bigint::ToBigInt;
use num::pow;

fn main() {

    let a: usize = 101;
    let b: usize = 101;
    let mut results = (2..a).map(|x| iter::repeat(x).take(b - 1).zip(2..b).collect::<Vec<(usize, usize)>>())
        .collect::<Vec<Vec<_>>>()
        .iter().flat_map(|elem| elem.iter().map(|sub| pow(sub.0.to_bigint().unwrap(), (sub.1))).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    results.sort();
    results.dedup();

    println!("{:?}", results.len());

}
