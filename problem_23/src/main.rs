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

    let ab_nums = (1..28123)
        .filter(|&x| proper_divs_sum(x) > x)
        .collect::<Vec<usize>>();

    let mut vec2: HashSet<usize> = HashSet::new();
    for elem in &ab_nums {
        for elem2 in &ab_nums {
            let result = elem + elem2;
            if result < 28123 {
                vec2.insert(result);
            }
        }
    }

    let no_ab_nums_sum = (1..28123)
        .filter(|x| !vec2.contains(x))
        .collect::<Vec<usize>>()
        .iter()
        .fold(0, |acc, item| acc + item);

    println!("{:?}", no_ab_nums_sum);
}
