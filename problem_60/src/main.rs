use std::collections::{HashMap, HashSet};

fn main() {

    macro_rules! set {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_set = HashSet::new();
                $(
                    temp_set.insert($x);
                )*
                temp_set
            }
        };
    }


    let mut concats = HashMap::new();


    fn is_prime(n: usize) -> bool {

        match n {
            1 => return false,
            2 | 3 => return true,
            _ => ()
        }

        match n % 2 {
            0 => return false,
            _ => ()
        }

        match n % 3 {
            0 => return false,
            _ => ()
        }

        let mut i = 5;
        loop {
            match i * i > n {
                true => break,
                false => {
                    if n % i == 0 || n % (i + 2) == 0 {
                        return false;
                    }
                }
            }
            i = i + 6;
        }
        true
    }

    fn concat_primes(n1: usize, n2: usize) -> bool {

        let n1s = n1.to_string();
        let n2s = n2.to_string();

        let p1 = (n1s.clone() + &n2s).parse::<usize>().unwrap();

        if !is_prime(p1) {
            return false;
        }

        let p2 = (n2s.clone() + &n1s).parse::<usize>().unwrap();

        if !is_prime(p2) {
            return false;
        }
        true
    }

    fn concat_primes_vec(primes: HashSet<usize>) -> bool {

        let sprimes = primes
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        for (i, n1s) in sprimes.iter().enumerate() {
            for n2s in (&sprimes[i+1..]).iter() {
                let p1 = (n1s.clone() + &n2s).parse::<usize>().unwrap();

                if !is_prime(p1) {
                    return false;
                }

                let p2 = (n2s.clone() + &n1s).parse::<usize>().unwrap();

                if !is_prime(p2) {
                    return false;
                }
            }
        }
        true
    }

    let primes = (2..)
        .filter(|&x| is_prime(x)).take(1200).collect::<Vec<usize>>();

    'outer: for p1 in primes.clone() {
        concats.insert(p1, Vec::new());
        for p2 in primes.clone() {
            if concat_primes(p1, p2) {
                concats.get_mut(&p1).unwrap().push(p2);
            }
        }
        for p2 in concats.get(&p1).unwrap() {
            for p3 in &concats.get(&p1).unwrap()[1..] {
                let p3ms = set![p1, *p2, *p3];
                if p3ms.len() != 3 || !concat_primes_vec(p3ms) {
                    continue;
                }
                for p4 in &concats.get(&p1).unwrap()[2..] {
                    let p4ms = set![p1, *p2, *p3, *p4];
                    if p4ms.len() != 4 || !concat_primes_vec(p4ms) {
                        continue;
                    }
                    for p5 in &concats.get(&p1).unwrap()[3..] {
                        let pms = set![p1, *p2, *p3, *p4, *p5];
                        if pms.len() != 5 {
                            continue;
                        }
                        if concat_primes_vec(pms) {
                            println!("{:?}", p1 + *p2 + *p3 + *p4 + *p5);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
}
