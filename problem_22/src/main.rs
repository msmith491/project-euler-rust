use std::io::prelude::*;
use std::fs::File;

fn main() {


    let mut s = String::new();

    let mut f = File::open("names.txt").unwrap();

    f.read_to_string(&mut s).unwrap();

    let mut vec = s.split("\",\"")
        .map(|x| x.replace("\"", ""))
        .collect::<Vec<String>>();

    vec.sort();

    fn letter_score(letter: char) -> usize {
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();

        for (index, l) in letters.enumerate() {
            if letter == l {
                return index + 1;
            }
        }
        panic!("We shouldn't get here");
    }

    fn name_score(name: &str) -> usize {

        name.chars()
            .map(|x| letter_score(x))
            .fold(0, |acc, item| acc + item)
    }

    let result = vec.iter()
        .map(|x| name_score(x))
        .enumerate()
        .collect::<Vec<(usize, usize)>>()
        .iter()
        .fold(0, |acc, item| acc + ((item.0 + 1) * item.1));

    println!("{:?}", result);
}
