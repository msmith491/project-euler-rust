extern crate range_check;

use range_check::Contains;

fn main() {

    fn is_prime(n: u64) -> bool {

        let mut primes: Vec<u64> = vec![1, 2, 3, 5, 7];
        if n < 1 {
            return false;
        } else if primes.contains(&n) {
            return true;
        } else if n % 2 == 0 || n % 3 == 0 {
            return false;
        } else {
            let mut i = 5;
            loop {
                if i * i > n {
                    break;
                } else {
                    if n % i == 0 || n % (i + 2) == 0 {
                        return false;
                    }
                }
                i = i + 6;
            }
        }
        true
    }

    fn sum_primes_below(n: u64) -> u64 {

        // let mut total: u64 = 0;

        // for elem in 2..n {
        //     if is_prime(elem) {
        //         total = total + elem;
        //     }
        // }
        // total

        (2..n).filter(|&x| is_prime(x)).fold(0, |acc, item| acc + item)
    }

    println!("{}", sum_primes_below(2000000));
}
