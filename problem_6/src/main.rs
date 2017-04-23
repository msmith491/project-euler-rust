macro_rules! timeit {
    ($func:expr) => ({
        let t1 = std::time::Instant::now();
        println!("{:?}", $func);
        let t2 = std::time::Instant::now().duration_since(t1);
        println!("{}", t2.as_secs() as f64 + t2.subsec_nanos() as f64 / 1000000000.00);
    })
}

fn main() {

    fn sum_of_squares(n: usize) -> usize {
        (1..(n + 1)).map(|x| x * x).fold(0, |acc, item| acc + item)
    }

    fn square_of_sums(n: usize) -> usize {
        (1..(n + 1)).fold(0, |acc, item| acc + item).pow(2)
    }

    fn s_s(n: usize) -> usize {
        square_of_sums(n) - sum_of_squares(n)
    }

    timeit!(s_s(100));
}
