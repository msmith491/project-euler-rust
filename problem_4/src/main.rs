macro_rules! timeit {
    ($func:expr) => ({
        let t1 = std::time::Instant::now();
        println!("{:?}", $func);
        let t2 = std::time::Instant::now().duration_since(t1);
        println!("{}", t2.as_secs() as f64 + t2.subsec_nanos() as f64 / 1000000000.00);
    })
}

fn main() {

    fn is_palindrome(n: usize) -> bool {
        n.to_string() == n.to_string().chars().rev().collect::<String>()
    }

    fn pal_prod() -> usize {
        (800..999)
            .rev()
            .map(|x| {
                (800..999)
                    .rev()
                    .map(|y| x * y)
                    .filter(|&z| is_palindrome(z as usize))
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap()
    }

    timeit!(pal_prod());
}
