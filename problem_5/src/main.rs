#![feature(step_by)]

fn main() {

    fn even_divisible(n: usize, h: usize) -> bool {
        for elem in (2..h + 1).rev() {
            if n % elem != 0 {
                return false;
            }
        }
        true
    }

    let max = 20;

    let result: usize = (max..)
        .step_by(max)
        .filter(|&x| even_divisible(x as usize, max))
        .take(1)
        .collect::<Vec<_>>()[0];

    println!("{:?}", result);
}
