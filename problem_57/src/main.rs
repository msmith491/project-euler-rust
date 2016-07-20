extern crate num;

use num::bigint::{BigInt, ToBigInt};
use std::fmt;

fn main() {

    #[derive(Clone)]
    struct Fraction {
        numerator: BigInt,
        denominator: BigInt
    }

    impl Fraction {
        fn new(numerator: BigInt, denominator: BigInt) -> Fraction {
            Fraction { numerator: numerator, denominator: denominator}
        }

        fn num_gt_dem(&self) -> bool {
            let num_len = self.numerator
                .to_string()
                .chars()
                .collect::<Vec<_>>()
                .len();
            let dem_len = self.denominator
                .to_string()
                .chars()
                .collect::<Vec<_>>()
                .len();
            num_len > dem_len
        }

    }

    impl fmt::Debug for Fraction {

        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Fraction {{ {}, {} }}",
                   self.numerator, self.denominator)
        }
    }

    impl Iterator for Fraction {

        type Item = Fraction;

        fn next(&mut self) -> Option<Fraction> {
            let f = calc_num_dem(self);
            self.numerator = f.numerator.clone();
            self.denominator = f.denominator.clone();
            Some(f)
        }
    }

    fn calc_num_dem(f: &Fraction) -> Fraction {
        let num1 = &(f.numerator) + (2.to_bigint().unwrap() * &(f.denominator));
        let dem1 = &num1 - &(f.denominator);
        Fraction::new(num1, dem1)
    }

    let mut f = Fraction::new(3.to_bigint().unwrap(), 2.to_bigint().unwrap());
    let nums = (0..1000)
        .map(|x| f.next().unwrap())
        .filter(|x| x.num_gt_dem())
        .collect::<Vec<Fraction>>();
    println!("{:?}", nums.len())
}
