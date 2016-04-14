use std::iter;
use std::collections::HashSet;

fn main() {

    static NUMS: &'static str = "123456789";

    // mc: multiplicand, mp: multiplier
    fn is_pandigital(mc: &usize, mp: usize) -> (bool, usize) {

        let pd = mc * mp;

        let s = [mc.to_string(),
                 mp.to_string(),
                 pd.to_string()].join("");

        if s.len() != 9 {
            return (false, 0);
        }

        (NUMS.chars().all(|x| s.contains(x)), pd)
    }

    fn get_valid_nums(n: usize) -> Vec<usize> {
        let nums = (2..n).map(|x| x
                                 .to_string()
                                 .chars()
                                 .collect::<Vec<char>>());

        let mut nums = nums.filter(|x| {
                                let y = x.len();
                                let mut mutx = x.clone();
                                mutx.sort();
                                mutx.dedup(); y == mutx.len()
                            })
                           .map(|x| x.into_iter()
                                     .collect::<String>()
                                     .parse::<usize>()
                                     .unwrap())
                           .collect::<Vec<usize>>();
        nums.sort();
        nums.dedup();
        nums
    }


    let valid = get_valid_nums(2000);
    let vlen = valid.len();
    let result = valid
        .iter()
        .map(|x| iter::repeat(x).take(vlen).zip(valid.clone()))
        .map(|elem| elem.filter(|&(mc, mp)| is_pandigital(mc, mp).0))
        .map(|elem| elem.map(|sub| (sub.0, sub.1, sub.0 * sub.1)).collect::<Vec<_>>())
        .filter(|elem| elem.len() != 0);

    let mut set: HashSet<usize> = HashSet::new();

    for elem in result {
        set.insert(elem[0].2);
    }

     println!("{:?}", set.iter().fold(0, |acc, item| acc + item));



}
