use std::collections::HashSet;

fn main() {

    fn calc_sequence(numerator: usize, denominator: usize) -> usize {
        let mut seq: HashSet<usize> = HashSet::new();
        let mut new_num = numerator;
        let mut length = 0;

        loop {
            if length >= denominator {
                break;
            }
            let mut remainder = new_num % denominator;
            new_num = remainder * 10;
            if seq.contains(&remainder) {
                break;
            }
            seq.insert(remainder);
            length += 1;
        }
        seq.len() + 1
    }

    println!("{}", (0..1000).rev().map(|x| calc_sequence(1, x)).max().unwrap());
}
