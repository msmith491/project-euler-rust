macro_rules! timeit {
    ($func:expr) => ({
        let t1 = std::time::Instant::now();
        println!("{:?}", $func);
        let t2 = std::time::Instant::now().duration_since(t1);
        println!("{}", t2.as_secs() as f64 + t2.subsec_nanos() as f64 / 1000000000.00);
    })
}

fn main() {

    fn num_divisors(n: usize) -> usize {
        match n == 1 {
            true => n,
            false => {
                (2..((n as f64).sqrt() as usize + 1))
                    .filter(|&x| n % x == 0)
                    .collect::<Vec<_>>()
                    .len() * 2 + 2
            }
        }
    }

    fn high_div_triag_num(divisors: usize) -> usize {
        (divisors..)
            .map(|x| (1..x).fold(0, |acc, item| acc + item))
            .filter(|&x| num_divisors(x) >= divisors)
            .take(1)
            .collect::<Vec<_>>()[0]
    }

    timeit!(high_div_triag_num(500));
}
