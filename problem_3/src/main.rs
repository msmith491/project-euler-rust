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

    // for elem in 1..100 {
    //     if is_prime(elem) {
    //         println!("{}", elem);
    //     }
    // }

    // println!("{}", is_prime(25));

    let num = 600851475143;
    // let num = 131952452;

    for elem in (2..((num / 2) as f64).sqrt() as i64 + 1).rev() {
        if (elem % 2 != 0 && num % elem == 0 && is_prime(elem)) || elem == 2 {
            println!("{}", elem);
            break;
        }
    }
}
