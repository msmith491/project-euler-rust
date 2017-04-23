macro_rules! timeit {
    ($func:expr) => ({
        let t1 = std::time::Instant::now();
        println!("{:?}", $func);
        let t2 = std::time::Instant::now().duration_since(t1);
        println!("{}", t2.as_secs() as f64 + t2.subsec_nanos() as f64 / 1000000000.00);
    })
}

fn main() {

    fn is_prime(n: usize) -> bool {

        let primes: Vec<usize> = vec![1, 2, 3, 5, 7];
        match n < 1 || n % 2 == 0 || n % 3 == 0 ||
              primes.iter().filter(|&x| *x == n).collect::<Vec<_>>().len() > 0 {
            true => return false,
            _ => {
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
        }
        true
    }


    fn largest_prime_factor(num: usize) -> usize {
        (2..((num / 2) as f64).sqrt() as usize + 1)
            .rev()
            .filter(|&x| x % 2 != 0 && num % x == 0 && is_prime(x) || x == 2)
            .take(1)
            .collect::<Vec<usize>>()[0]
    }

    timeit!(largest_prime_factor(600851475143))
}
