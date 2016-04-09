extern crate range_check;

use range_check::Contains;

fn main() {

    fn is_prime(n: i64) -> bool {

        let mut primes: Vec<i64> = vec![1, 2, 3, 5, 7];
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

    let mut found = 5;

    for num in 12.. {
        if is_prime(num) {
            found = found + 1;
        }
        if found >= 10001 {
            println!("{}", num);
            break;
        }
    }
}
