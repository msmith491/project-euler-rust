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
            false => {
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

    fn get_prime(n: usize) -> usize {
        (12..)
            .filter(|&x| is_prime(x))
            .take(n - 5)
            .max()
            .unwrap()
    }

    timeit!(get_prime(10001))
}
