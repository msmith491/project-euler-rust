use std::collections::HashSet;

fn main() {

    struct PentNums {
        n: usize,
        curr: usize
    }

    impl PentNums {

        fn new(index: usize) -> PentNums {
            if index == 0 {
                return PentNums{n: index, curr: 0};
            }
            PentNums{n: index, curr: PentNums::get(index)}
        }

        fn get(index: usize) -> usize {
            (index * ((index * 3) - 1)) / 2
        }
    }

    impl Iterator for PentNums {

        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            self.n += 1;
            self.curr = (self.n * ((self.n * 3) - 1)) / 2;
            Some(self.curr)
        }
    }

    fn is_pent(num: usize, mut h: &mut HashSet<usize>) -> bool {

        if h.contains(&num) {
            return true
        }

        let mut p = PentNums::new(h.len());
        for elem in p {
            if num == elem {
                h.insert(num);
                return true;
            } else if elem > num {
                return false;
            }
        }
        false
    }

    // assumes a and b are pentagonal
    fn sum_diff_pent(a: usize, b: usize, mut h: &mut HashSet<usize>) -> bool {

        if a >= b {
            return false;
        }

        let e = a + b;
        // println!("{:?}", e);

        if !is_pent(e, &mut h) {
            return false;
        }


        let d = b - a;
        // println!("{:?}", d);

        if !is_pent(d, &mut h) {
            return false;
        }

        true
    }


    let mut pA = PentNums::new(8)
        .take(2500)
        .collect::<Vec<_>>();

    let mut h: HashSet<usize> = HashSet::new();
    for num in pA.clone() {
        h.insert(num);
    }

    'outer: for curr in pA {

        // println!("{:?}", curr);
        let mut pB = PentNums::new(4)
            .take(2500)
            .collect::<Vec<_>>();

        for elem in pB {

            // println!("{:?}", elem);
            if elem >= curr {
                continue 'outer;
            }

            if sum_diff_pent(elem, curr, &mut h) {
                println!("{:?}", curr - elem);
                break 'outer;
            }

        }
    }

}
