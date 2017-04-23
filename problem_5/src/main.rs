#![feature(step_by)]

macro_rules! timeit {
    ($func:expr) => ({
        let t1 = std::time::Instant::now();
        println!("{:?}", $func);
        let t2 = std::time::Instant::now().duration_since(t1);
        println!("{}", t2.as_secs() as f64 + t2.subsec_nanos() as f64 / 1000000000.00);
    })
}

fn main() {

    fn even_divisible(n: usize, h: usize) -> bool {
        for elem in (2..h + 1).rev() {
            if n % elem != 0 {
                return false;
            }
        }
        true
    }

    fn smallest_multiple(max: usize) -> usize {
        (max..)
            .step_by(max)
            .filter(|&x| even_divisible(x as usize, max))
            .take(1)
            .collect::<Vec<_>>()[0]
    }

    timeit!(smallest_multiple(20));
}
