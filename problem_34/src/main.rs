fn main() {

    fn sum_factorial(n: usize) -> usize {

        let digits = n.to_string()
                      .chars()
                      .map(|x| x.to_digit(10).unwrap() as usize)
                      .collect::<Vec<usize>>();

        return digits.iter()
                     .map(|&x| (1..(x + 1)).fold(1, |acc, item| acc * item))
                     .fold(0, |acc, item| acc + item);
    }

    // println!("{:?}", sum_factorial(145));

     println!("{:?}", (3..100000).filter(|&x| x == sum_factorial(x))
                                  .fold(0, |acc, item| acc + item));

}
