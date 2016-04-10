use std::collections::HashSet;

fn main() {

    fn proper_divs_sum(n: usize) -> usize {
        let mut vec: Vec<usize> = vec![1];
        for div in 2..(n / 2) {
            if n % div == 0 {
                vec.push(div);
                vec.push(n / div);
            }
        }
        vec.into_iter()
            .collect::<HashSet<usize>>()
            .into_iter()
            .collect::<Vec<usize>>()
            .iter()
            .fold(0, |acc, item| acc + item)
    }

    // fn is_amicable_pair(n1: usize, n2: usize) -> bool {
    //     if n1 != n2 && proper_divs_sum(n1) == n2 && proper_divs_sum(n2) == n1 {
    //         return true;
    //     }
    //     false
    // }

    fn fast_amicable_pair(n1: usize, n2: usize) -> bool {
        // This one assumes we already know d(n1) == n2
        // so we don't calculate the proper divs twice for n1
        if n1 != n2 && proper_divs_sum(n2) == n1 {
            return true;
        }
        false
    }

    let mut vec: Vec<usize> = Vec::new();

    for n1 in 1..10000 {
        // There's probably a good reason to not iterate over
        // all 10000 nums since we have to dedup the list, but
        // this is fast enough as it is
        let n2 = proper_divs_sum(n1);
        if fast_amicable_pair(n1, n2) {
            vec.push(n1);
            vec.push(n2);
        }
    }

    let dedup = vec
        .into_iter()
        .collect::<HashSet<usize>>()
        .into_iter()
        .collect::<Vec<usize>>()
        .iter()
        .fold(0, |acc, item| acc + item);

    println!("{:?}", dedup);
}
