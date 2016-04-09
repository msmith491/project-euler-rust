fn main() {

    fn is_palindrome(n: i64) -> bool {
        let s = n.to_string();
        let rev: String = n.to_string().chars().rev().collect();
        s == rev
    }

    // println!("{}", is_palindrome(1234321));

    let mut largest = 0;

    for x in (800..999).rev() {
        for y in (800..999).rev() {
            let mult = x * y;
            if is_palindrome(mult) && mult > largest {
                println!("{}", mult);
                largest = mult
            }
        }
    }
}
