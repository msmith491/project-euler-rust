use std::collections::HashSet;

fn main() {

    fn is_prime(n: usize) -> bool {

        match n {
            1 => return false,
            2 | 3 => return true,
            _ => ()
        }

        match n % 2 {
            0 => return false,
            _ => ()
        }

        match n % 3 {
            0 => return false,
            _ => ()
        }

        let mut i = 5;
        loop {
            match i * i > n {
                true => break,
                false => {
                    if n % i == 0 || n % (i + 2) == 0 {
                        return false;
                    }
                }
            }
            i = i + 6;
        }
        true
    }

    // fn prime_factors(n: usize, primes: &HashSet<usize>) -> HashSet<usize> {

    //     let mut curr = n;
    //     let mut prime_factors = HashSet::new();

    //     loop {
    //         if curr % 2 != 0 {
    //             break;
    //         }
    //         curr = curr / 2;
    //         prime_factors.insert(2);
    //     }

    //     let mut divisor = 3;
    //     loop {
    //         if primes.contains(&curr) {
    //             if curr < n {
    //                 prime_factors.insert(curr);
    //             }
    //             break;
    //         }
    //         if curr % divisor == 0 {
    //             if primes.contains(&divisor) {
    //                 prime_factors.insert(divisor);
    //             }
    //         }
    //         if divisor >= curr {
    //             break;
    //         }
    //         divisor += 2;
    //     }
    //     prime_factors
    // }
    fn prime_factors(n: usize, primes: &HashSet<usize>) -> HashSet<&usize> {

        let mut hs = HashSet::new();

        for p in primes {
            if n % p == 0 {
                hs.insert(p);
            }
        }
        hs
    }

    let primes = (2..)
        .filter(|&x| is_prime(x))
        .take(10000)
        .collect::<HashSet<usize>>();

    let result = ((2..)
                  .filter(|&x| is_prime(x))
                  .take(4)
                  .fold(0, |a, i| a + i)
                  ..)
        .filter(|&x| x % 2 != 0)
        .map(|x| (x, prime_factors(x, &primes)))
        .filter(|y| y.1.len() == 4)
        .map(|y| y.0)
        .filter(|x| {
            // println!("{:?}", x);
            for t in (x+1)..(x+4) {
                if prime_factors(t, &primes).len() != 4 {
                    return false;
                }
            }
            true
        })
        .take(1)
        .collect::<Vec<_>>();

    println!("{:?}", result[0]);
}
