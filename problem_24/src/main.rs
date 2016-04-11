use std::collections::HashSet;

fn main() {


    fn permutations(num: usize) -> Vec<Vec<usize>> {
        let mut a = (0..num).collect::<Vec<usize>>();
        let mut n = a.len();
        let mut result: Vec<Vec<usize>> = Vec::new();

        fn perm(n: usize, v: &mut Vec<usize>, r: &mut Vec<Vec<usize>>) {
            if n == 1 {
                // println!("{:?}", v);
                r.push(v.clone());
            } else {
                for i in 0..(n - 1) {
                    perm(n - 1, v, r);

                    if n % 2 == 0 {
                        v.swap(i, n - 1);
                    } else {
                        v.swap(0, n - 1);
                    }
                }
                perm(n - 1, v, r);
            }

        }
        perm(n, &mut a, &mut result);
        result
    }

    // fn permutations_hash(num: usize) -> HashSet<Vec<usize>> {
    //     let mut a = (0..num).collect::<Vec<usize>>();
    //     let mut n = a.len();
    //     let mut hs: HashSet<Vec<usize>> = HashSet::new();

    //     fn perm(n: usize, v: &mut Vec<usize>, s: &mut HashSet<Vec<usize>>) {
    //         if n == 1 {
    //             // println!("{:?}", v);
    //             s.insert(v.clone());
    //         } else {
    //             for i in 0..(n - 1) {
    //                 perm(n - 1, v, s);

    //                 if n % 2 == 0 {
    //                     v.swap(i, n - 1);
    //                 } else {
    //                     v.swap(0, n - 1);
    //                 }
    //             }
    //             perm(n - 1, v, s);
    //         }

    //     }
    //     perm(n, &mut a, &mut hs);
    //     hs
    // }

    let mut perms = permutations(10);
        // .into_iter()
        // .collect::<Vec<Vec<usize>>>();

    perms.sort();
    println!("{:?}", perms.get(1000000 - 1).unwrap())

}
