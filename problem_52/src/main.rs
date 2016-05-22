fn main() {

    fn same_digits(n1: usize, n2: usize) -> bool {
        let s1 = n1.to_string();
        let s2 = n2.to_string();
        if s1.len() != s2.len() {
            return false;
        }
        s1
            .chars()
            .all(|x| s2.contains(&x.to_string()))
    }

    fn check_num(n: usize, f: usize) -> bool {
        for f in 2..(f + 1) {
            if !same_digits(n, n * f) {
                return false;
            }
        }
        true
    }

    let s = (10..)
        .filter(|&x| check_num(x, 6))
        .take(1)
        .collect::<Vec<_>>();

    println!("{:?}", s[0]);
}
