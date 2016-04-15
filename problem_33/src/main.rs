use std::iter;

fn main() {

    fn check(n: usize, i: usize, d: usize) -> bool {

        if !(n < d && d < i) {
            return false;
        }
        d * ((10 * n) + i) == n * ((10 * i) + d)
    }

    let mut num_result = 1;
    let mut dem_result = 1;
    for n in 1..100 {
        for d in 1..100 {
            for i in 1..10 {
                if check(n, i, d) {
                    // println!("{:?}", [n, d, i]);
                    num_result *= n;
                    dem_result *= d;
                }

            }
        }
    }
    println!("{:?}", dem_result/num_result);
}
