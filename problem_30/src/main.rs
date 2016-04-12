fn main() {

    fn power_digits(n: usize, l: usize) -> bool {

        let s = n.to_string().chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<_>>();

        let result = s.iter().map(|x| x.pow(l as u32)).collect::<Vec<_>>()
            .iter().fold(0, |acc, item| acc + item);
        n == (result as usize)
    }

    // println!("{}", power_digits(1635, 4));

    let nums = (2..1000000).filter(|&x| power_digits(x, 5)).collect::<Vec<_>>();

    println!("{:?}", nums.iter().fold(0, |acc, item| acc + item));
}
