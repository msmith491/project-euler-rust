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

    let mut comps = (4..)
        .filter(|&x| x % 2 != 0)
        .filter(|&x| !is_prime(x))
        .take(3000)
        .collect::<HashSet<usize>>();

    let primes = (2..)
        .filter(|&x| is_prime(x))
        .take(3000)
        .collect::<Vec<usize>>();

    let squares = (1..)
        .map(|x| (x as usize).pow(2 as u32) * 2)
        .take(3000)
        .collect::<Vec<usize>>();

    'outer: for square in &squares {
        for prime in &primes {
            let result = prime + square;
            if result % 2 != 0 &&
               comps.contains(&result) {
                   comps.remove(&result);
            }
        }
    }

    println!("{:?}", comps.iter().min().unwrap());

}
