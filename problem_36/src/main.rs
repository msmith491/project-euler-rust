fn main() {

    fn is_palindrome(n: usize) -> bool {
        let rev = n.to_string().chars().rev().collect::<String>();
        n.to_string() == rev
    }

    // println!("{:?}", is_palindrome(1000));
    fn to_binary(n: usize) -> usize {

        let mut factor: u32 = 0;
        let mut rem = 0;
        let mut curr = n;
        let mut result = 0;
        let base = 2;
        loop {
            rem = curr % 2;
            curr = curr / 2;
            result += 10usize.pow(factor) * rem;
            factor += 1;
            if curr <= 0 {
                break;
            }
        }
        result
    }

    let nums = (1..1000000).filter(|&x| {
        if !is_palindrome(x) {
            return false;
        } else if !is_palindrome(to_binary(x)) {
            return false;
        }
        true
    }).collect::<Vec<usize>>();

    println!("{:?}", nums.iter().fold(0, |acc, item| acc + item));
}
