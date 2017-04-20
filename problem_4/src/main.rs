fn main() {

    fn is_palindrome(n: usize) -> bool {
        n.to_string() == n.to_string().chars().rev().collect::<String>()
    }

    let result = (800..999)
        .rev()
        .map(|x| {
            (800..999)
                .rev()
                .map(|y| x * y)
                .filter(|&z| is_palindrome(z as usize))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap();

    println!("{}", result);
}
