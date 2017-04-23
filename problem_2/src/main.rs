macro_rules! timeit {
    ($func:expr) => ({
        let t1 = std::time::Instant::now();
        println!("{:?}", $func);
        let t2 = std::time::Instant::now().duration_since(t1);
        println!("{}", t2.as_secs() as f64 + t2.subsec_nanos() as f64 / 1000000000.00);
    })
}

fn main() {

    fn fib(n: usize) -> usize {
        match n <= 3 {
            true => n,
            false => return fib(n - 1) + fib(n - 2),
        }
    }

    fn even_fibs(n: usize) -> usize {
        (1..)
            .map(|x| fib(x))
            .take_while(|&x| x <= n as usize)
            .filter(|&x| x % 2 == 0)
            .fold(0, |acc, item| acc + item)
    }

    timeit!(even_fibs(4000000))
}
