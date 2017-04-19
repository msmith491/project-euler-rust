fn main() {

    fn fib(n: usize) -> usize {
        match n <= 3 {
            true => n,
            false => return fib(n - 1) + fib(n - 2),
        }
    }

    let sum = (1..)
        .map(|x| fib(x))
        .take_while(|&x| x <= 4000000 as usize)
        .filter(|&x| x % 2 == 0)
        .fold(0, |acc, item| acc + item);

    println!("{}", sum);
}
