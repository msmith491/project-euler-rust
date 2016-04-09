fn main() {

    fn sum_of_squares(n: i64) -> i64 {
        (1..(n + 1)).map(|x| x * x).fold(0, |acc, item| acc + item)
    }

    fn square_of_sums(n: i64) -> i64 {
        (1..(n + 1)).fold(0, |acc, item| acc + item).pow(2)
    }

    println!("{}", square_of_sums(100) - sum_of_squares(100));
}
