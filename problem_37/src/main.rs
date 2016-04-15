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

    fn truncate_right(num: usize) -> usize {

        let mut iterable = num.to_string().chars().collect::<Vec<char>>();
        iterable.pop();
        iterable.into_iter().collect::<String>().parse::<usize>().unwrap()
    }

    fn truncate_left(num: usize) -> usize {

        let mut iterable = num.to_string().chars().collect::<Vec<char>>();
        iterable.remove(0);
        iterable.into_iter().collect::<String>().parse::<usize>().unwrap()
    }

    fn truncated_prime(n: usize) -> bool {

        if n < 10 {
            return false;
        }

        if !is_prime(n) {
            return false;
        }

        let mut curr = n;
        loop {
            curr = truncate_right(curr);

            if !is_prime(curr) {
                return false
            }

            if curr < 10 {
                break;
            }
        }

        let mut curr = n;
        loop {
            curr = truncate_left(curr);

            if !is_prime(curr) {
                return false
            }

            if curr < 10 {
                break;
            }
        }
        true
    }

    let result = (11..)
        .filter(|&x| truncated_prime(x))
        .take(11)
        .collect::<Vec<usize>>();
    println!("{:?}", result.iter().fold(0, |a, i| a + i));
}
