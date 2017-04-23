macro_rules! timeit {
    ($func:expr) => ({
        let t1 = std::time::Instant::now();
        println!("{:?}", $func);
        let t2 = std::time::Instant::now().duration_since(t1);
        println!("{}", t2.as_secs() as f64 + t2.subsec_nanos() as f64 / 1000000000.00);
    })
}

fn main() {

    fn is_pythagorean_triplet(a: usize, b: usize, c: usize) -> bool {
        a < b && b < c && a.pow(2) + b.pow(2) == c.pow(2)
    }

    fn test_triplets() -> usize {
        for a in 3..1000 {
            for b in a..1000 {
                for c in b..1000 {
                    if a + b + c == 1000 {
                        if is_pythagorean_triplet(a, b, c) {
                            return a * b * c;
                        }
                    }
                }
            }
        }
        0
    }

    timeit!(test_triplets());
}
