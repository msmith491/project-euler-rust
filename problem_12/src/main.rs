fn main() {

    fn is_triangle_num(n: u64) -> bool {

        let (mut current, mut index) = (0, 0);

        loop {
            if current == n {
                return true;
            }
            current = current + index;
            if current > n {
                break;
            }
            index = index + 1;
        }
        false
    }

    fn num_divisors(n: u64) -> u64 {
        if n == 1 {
            return 1;
        }
        let mut count = 0;
        for div in 2..((n as f64).sqrt() as u64 + 1) {
            if n % div == 0 {
                count = count + 1;
            }
        }
        (count * 2) + 2
    }

    // println!("{:?}", (1..1000000)
    //          .filter(|&x| is_triangle_num(x))
    //          .map(|x| num_divisors(x))
    //          .collect::<Vec<u64>>());

    let nums = (1..).map(|x| (1..x).fold(0, |acc, item| acc + item));

    for num in nums {

        let divs = num_divisors(num);

        if divs >= 500 {
            println!("{}", num);
            break;
        }
    }
}
