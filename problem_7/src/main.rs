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

    let result = (12..)
        .filter(|&x| is_prime(x))
        .take(10001 - 5)
        .max()
        .unwrap();

    println!("{:?}", result);

}
