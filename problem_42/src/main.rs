use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn main() {

    static LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    fn gen_triangle_nums(n: usize, s: &mut HashSet<usize>) {
        for elem in 1..n {
            let t_num = (elem * (elem + 1)) / 2;
            s.insert(t_num);
        }
    }

    fn letter_score(a: char) -> usize {
        LETTERS.find(a).unwrap() + 1
    }

    fn word_score(w: &str) -> usize {
        w.chars().map(|x| letter_score(x)).fold(0, |a, i| a + i)
    }

    // Generate triangle number set
    let mut t_nums: HashSet<usize> = HashSet::new();
    gen_triangle_nums(1000, &mut t_nums);

    // Read in file and format contents
    let mut f = File::open("words.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    let words = s.split("\",\"").map(|x| x.replace("\"", "")).collect::<Vec<String>>();

    let result = words
        .iter()
        .map(|word| word_score(&word))
        .filter(|&x| t_nums.contains(&x))
        .collect::<Vec<usize>>();
    println!("{:?}", result.len());
}
