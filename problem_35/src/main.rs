use std::collections::HashSet;

fn main() {

    fn is_prime(n: usize) -> bool {

        if n < 1 {
            return false;
        } else if n == 2 || n == 3 {
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

    fn rotate(num: usize) -> usize {

        let mut iterable = num.to_string().chars().collect::<Vec<char>>();
        let item = iterable.pop().unwrap();
        iterable.insert(0, item);
        iterable.into_iter().collect::<String>().parse::<usize>().unwrap()
    }

    let mut primes: HashSet<usize> = HashSet::with_capacity(500);

    let result = (2..1000000)
        .filter(|&x| {
            let mut newx = x;
            loop {
                if !primes.contains(&newx) && !is_prime(newx) {
                    return false;
                } else {
                    primes.insert(newx);
                    newx = rotate(newx);
                    if newx == x {
                        return true;
                    }
                }
            }
        }).collect::<Vec<usize>>();

    println!("{:?}", result.len());

}
