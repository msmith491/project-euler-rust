fn main() {

    // Helper benchmarking function
    // Definitely benchmark with --release flag (0.05 seconds vs 2.3 seconds)
    fn timeit(t1: std::time::Instant) -> f64 {
        let t2 = std::time::Instant::now().duration_since(t1);
        t2.as_secs() as f64 + t2.subsec_nanos() as f64 / 1000000000.00
    }

    fn is_pythagorean_triplet(a: usize, b: usize, c: usize) -> bool {
        a < b && b < c && a.pow(2) + b.pow(2) == c.pow(2)
    }

    let t1 = std::time::Instant::now();
    for a in 3..1000 {
        for b in a..1000 {
            for c in b..1000 {
                if a + b + c == 1000 {
                    if is_pythagorean_triplet(a, b, c) {
                        println!("{}", a * b * c);
                        println!("{}", timeit(t1));
                        return;
                    }
                }
            }
        }
    }
}
