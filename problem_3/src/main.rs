fn main() {

    fn is_prime(n: usize) -> bool {

        let primes: Vec<usize> = vec![1, 2, 3, 5, 7];
        match n < 1 || n % 2 == 0 || n % 3 == 0 || primes.iter().filter(|&x| *x == n).collect::<Vec<_>>().len() > 0 {
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
            },
        }
        true
    }


    let num = 600851475143;

    let result = (2..((num / 2) as f64).sqrt() as usize + 1)
        .rev()
        .filter(|&x| x % 2 != 0 && num % x == 0 && is_prime(x) || x == 2)
        .take(1)
        .collect::<Vec<usize>>()[0];
    println!("{:?}", result);
}
