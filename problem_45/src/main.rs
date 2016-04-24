use std::collections::HashSet;

fn main() {

    struct TriNums {
    }

    struct PentNums {
    }

    struct HexNums {
    }

    impl TriNums {

        fn get(index: usize) -> usize {
            (index * (index + 1)) / 2
        }
    }

    impl PentNums {

        fn get(index: usize) -> usize {
            (index * ((index * 3) - 1)) / 2
        }
    }

    impl HexNums {

        fn get(index: usize) -> usize {
            index * ((index * 2) - 1)
        }
    }

    fn get_nums(i: usize) -> (usize, usize, usize) {

        (TriNums::get(i),
         PentNums::get(i),
         HexNums::get(i))
    }

    // println!("{:?}", (1..10).map(|x| get_nums(x)).collect::<Vec<_>>());

    let (mut th, mut ph, mut hh) = (
        HashSet::new(), HashSet::new(), HashSet::new());

    for n in 2.. {
        let (tn, pn, hn) = get_nums(n);

        th.insert(tn);
        ph.insert(pn);
        hh.insert(hn);

        if ph.contains(&tn) && hh.contains(&tn) && n != 285 {
            println!("{:?}", tn);
            break;
        }
    }
}
