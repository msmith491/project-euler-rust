fn main() {

    fn is_pythagorean_triplet(a: i64, b: i64, c: i64) -> bool {
        if a < b && b < c {
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return true;
            }
        }
        false
    }

    // println!("{}", is_pythagorean_triplet(3, 4, 5));

    'out: for a in 3..1000 {
        for b in 4..1000 {
            for c in 5..1000 {
                if a + b + c == 1000 {
                    if is_pythagorean_triplet(a, b, c) {
                        // println!("{}, {}, {}", a, b, c);
                        println!("{}", a * b * c);
                        break 'out;
                    }
                }
            }
        }
    }
}
