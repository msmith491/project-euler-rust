fn main() {

    fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        } else {
            return fib(n - 1) + fib(n - 2);
        }
    }

    let mut fib_seq = Vec::new();
    let mut index = 1;
    loop {
        let current_fib = fib(index);
        if current_fib <= 4000000 {
            fib_seq.push(fib(index));
        } else {
            break;
        }
        index = index + 1;
    }

    let sum: i32 = fib_seq
        .into_iter()
        .filter(|&x| x % 2 == 0)
        .fold(0, |acc, item| acc + item);

    println!("{}", sum);
        // .filter(|x: i32| x % 2 == 0)
}
