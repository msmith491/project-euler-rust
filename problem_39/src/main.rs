#![feature(step_by)]

use std::collections::HashMap;

fn main() {

    let mut hm = HashMap::new();

    fn check_func(a: usize, p: usize) -> bool {
        ((p.pow(2) - (2 * p * a))) % ((2 * p) - (2 * a)) == 0
    }
    // Perimiter of integral right triangles is always even
    for p in (4..1001).step_by(2) {
        for a in 1..(p/3) {
            if check_func(a, p) {
                let entry = hm.entry(p).or_insert(0);
                *entry = *entry + 1;
            }
        }
    }

    let mut vec = hm
             .into_iter()
             .collect::<Vec<(usize, usize)>>();

    vec.sort_by_key(|elem| elem.1);

    println!("{:?}", vec.last().unwrap().0);

}
