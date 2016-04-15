fn main() {

    static NUMS: &'static str = "123456789";

    fn is_pandigital(n: usize) -> bool {

        let s = n.to_string();

        if s.len() != 9 {
            return false;
        }

        NUMS.chars().all(|x| s.contains(x))
    }

    fn n_concat(n: usize) -> usize {
        let s = n.to_string();

        let mut result = String::new();
        for index in 1.. {
            let num = (index * n).to_string();
            result = result + &num;

            if result.len() >= 8 {
                return result.parse::<usize>().unwrap();
            }
        }
        0
    }

    let result = (1..)
        .map(|x| n_concat(x))
        .filter(|&x| is_pandigital(x))
        .take(17)
        .max()
        .unwrap();

    println!("{:?}", result);
}
