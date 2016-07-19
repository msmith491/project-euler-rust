extern crate num;

use num::bigint::{ToBigInt,BigInt};

fn main() {

    // Lychrel numbers

    fn is_palindrome(n: &str) -> bool {
        let rev = n.chars().rev().collect::<String>();
        n.to_string() == rev
    }

    fn is_lychrel_num(n: usize) -> bool {
        let mut current = n.to_string();
        for elem in 0..50 {
            let rev = current
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<BigInt>()
                .unwrap();
            let pal = (current.parse::<BigInt>().unwrap() + rev).to_string();
            if is_palindrome(&pal) {
                return false
            } else {
                current = pal
            }
        }
        true
    }

    let result = (1..10000)
        .filter(|&x| is_lychrel_num(x))
        .collect::<Vec<_>>();

    println!("{:?}", result.len());
}
