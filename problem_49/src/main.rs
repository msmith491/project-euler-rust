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

    fn is_permutation(a: usize, b: usize) -> bool {

        if a == b {
            return false;
        }

        let s1 = a.clone().to_string();
        let s2 = b.clone().to_string();

        if s1.len() != s2.len() {
            return false;
        }

        let s2b = s2.chars().collect::<HashSet<char>>();

        let r1 = s1.chars().all(|x| s2b.contains(&x));

        let s1b = s1.chars().collect::<HashSet<char>>();

        let r2 = s2.chars().all(|x| s1b.contains(&x));

        r1 && r2
    }

    let primes = (1000..10000)
        .filter(|&x| is_prime(x))
        .collect::<Vec<usize>>();

    let mut result = Vec::new();

    let filtered = primes
        .iter()
        .filter(|&x| {
            let primes2 = primes
                .clone()
                .into_iter()
                .filter(|y| y > x);
            for p in primes2 {
                let diff = p - x;
                let k = p + diff;
                if is_prime(k) &&
                   k < 10000 &&
                   is_permutation(*x, p) &&
                   is_permutation(*x, k) {
                    result.push([*x, p, k]);
                    return true;
                }
            }
            false
        })
        .collect::<Vec<_>>();

    println!("{:?}", result
             .last()
             .unwrap()
             .iter()
             .fold(String::new(), |a, i| a + &i.to_string())
             .parse::<usize>()
             .unwrap());
    // println!("{:?}", is_permutation(6679, 6793));
}
