macro_rules! timeit {
    ($func:expr) => ({
        let t1 = std::time::Instant::now();
        println!("{:?}", $func);
        let t2 = std::time::Instant::now().duration_since(t1);
        println!("{}", t2.as_secs() as f64 + t2.subsec_nanos() as f64 / 1000000000.00);
    })
}

fn main() {

    fn mult_3_5(n: usize) -> usize {
        (0..n)
            .filter(|x| x % 3 == 0 || x % 5 == 0)
            .fold(0, |acc, item| acc + item)
    }

    timeit!(mult_3_5(1000));
}
