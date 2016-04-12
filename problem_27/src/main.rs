use std::collections::HashSet;
use std::iter;

fn main() {

    // let mut primes: HashSet<isize> = vec![1, 2, 3, 5, 7].into_iter().collect();

    fn is_prime(n: isize) -> bool {

        if n < 1 {
            return false;
        } else if n % 2 == 0 || n % 3 == 0 {
            return false;
        } else {
            let mut i = 5;
            loop {
                if i * i > n {
                    break;
                } else {
                    if n % i == 0 || n % (i + 2) == 0 {
                        return false;
                    }
                }
                i = i + 6;
            }
        }
        // p.insert(n);
        true
    }

    // println!("{:?}", (0..)
    //          .filter(|&x| is_prime(x, &mut primes))
    //          .take(10)
    //          .collect::<Vec<isize>>());

    // println!("{:?}", primes);

    struct Formula (isize, isize, isize);

    impl Iterator for Formula {
        type Item = isize;

        fn next(&mut self) -> Option<isize> {
            self.0 = self.0 + 1;
            let result = self.0.pow(2) + (self.1 * self.0) + self.2;
            Some(result)
        }
    }

    let result = (-1000..1000).map(|x| iter::repeat(x).take(2000).zip(-1000..1000))
        .map(|x| x.map(|(a, b)| (a, b, Formula(0, a, b))))
        .map(|z| z.map(|(a, b, f)| (a, b, f.take_while(|&x| is_prime(x)).collect::<Vec<isize>>())))
        .map(|elem| elem.map(|(a, b, sub)| (a, b, sub.len() as isize)))
        .map(|elem| elem.collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();



    let mut largest = 0;
    let mut end_item: (isize, isize, isize) = (0, 0, 0);

    for item in result {
        for subitem in item {
            if subitem.2 >= largest {
                largest = subitem.2;
                end_item = subitem;
            }
        }
    }
    // println!("{:?}", largest);
    // println!("{:?}", end_item);
    println!("{:?}", end_item.0 * end_item.1);
}
