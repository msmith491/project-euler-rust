use std::char;

fn main() {

    fn is_prime(n: usize) -> bool {

        if n <= 1 {
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

    fn is_pandigital(n: usize, l: usize) -> bool {

        let mut nums = (1..(l + 1))
            .map(|x| char::from_digit(x as u32, 10).unwrap());

        let s = n.to_string();

        if s.len() != l {
            return false;
        }

        nums.all(|x| s.contains(x))
    }

    fn num_digits(n: usize) -> usize {
        ((n as f64).log10() as usize) + 1
    }

    let result = (2143..8000000)
        .filter(|&x| is_pandigital(x as usize, num_digits(x)) && is_prime(x))
        .collect::<Vec<usize>>();

    println!("{:?}", result.last().unwrap());
}
