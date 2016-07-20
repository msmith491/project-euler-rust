fn main() {

    // 37 36 35 34 33 32 31
    // 38 17 16 15 14 13 30
    // 39 18  5  4  3 12 29
    // 40 19  6  1  2 11 28
    // 41 20  7  8  9 10 27
    // 42 21 22 23 24 25 26
    // 43 44 45 46 47 48 49

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

    fn nth_side_length(n: usize) -> usize {
        (2 * n) - 1
    }

    fn diag1(n: usize) -> usize {
        (4 * n * n) - (2 * n) + 1
    }

    fn diag2(n: usize) -> usize {
        (4 * n * n) + (2 * n) + 1
    }

    fn diag3(n: usize) -> usize {
        (4 * n * n) + 1
    }

    fn diag4(n: usize) -> usize {
        let n = n + 1;
        (4 * n * n) - (4 * n) + 1
    }

    fn total_primes_for_level(n: usize, prev: (usize, usize)) -> (usize, usize) {
        let v = vec![diag1(n), diag2(n), diag3(n), diag4(n)];
        let curr = v.iter().filter(|&x| is_prime(*x)).collect::<Vec<_>>().len();
        let num = prev.0 + curr;
        let dem = (4 * n) + 1;
        (num, dem)
    }

    let mut prev = (0, 0);

    for n in 1.. {
        let (num, dem) = total_primes_for_level(n, prev);
        let curr = num as f64 / dem as f64;
        if curr < 0.1 {
            println!("{:?}", nth_side_length(n+1));
            break;
        }
        else {
            prev = (num, dem);
        }
    }
}
