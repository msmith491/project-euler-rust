fn main() {

    fn timeit(t1: std::time::Instant) -> f64 {
        let t2 = std::time::Instant::now().duration_since(t1);
        t2.as_secs() as f64 + t2.subsec_nanos() as f64 / 1000000000.00
    }

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

    fn sum_primes_below(n: usize) -> usize {

        (2..n).filter(|&x| is_prime(x as usize)).fold(0, |acc, item| acc + item)
    }

    let t1 = std::time::Instant::now();
    println!("{}", sum_primes_below(2000000));
    println!("{}", timeit(t1));
}
