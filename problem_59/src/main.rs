use std::io::prelude::*;
use std::fs::File;

fn main() {

    static alphabet: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
                                     'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                                     'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
                                     'y', 'z'];

    struct Password {
        word: String,
        index: isize
    }

    impl Password {

        fn new(word: String) -> Password {
            Password { word: word, index: -1}
        }
    }

    impl Iterator for Password {
        type Item = char;

        fn next(&mut self) -> Option<char> {
            self.index = (self.index + 1) % 3;
            self.word.chars().nth(self.index as usize)
        }
    }

    fn freq_analysis(s: String) {

    }

    let mut f = File::open("cipher.txt").unwrap();
    let mut s = String::new();
    let mut found = String::new();
    // Spaces are the most common character in English
    let mut highest_space = 0;
    let mut found_key = String::new();
    f.read_to_string(&mut s);
    s = s.replace("\n", "");
    for a in &alphabet {
        for b in &alphabet {
            for c in &alphabet {
                let key = vec![a.to_string(), b.to_string(), c.to_string()].join("").to_string();
                let mut p = Password::new(key.clone());
                let ascii = s
                    .split(",")
                    .map(|x| ((x.parse::<u8>().unwrap() ^ (
                                    p.next().unwrap() as u8)) as char).to_string())
                    .collect::<Vec<_>>();
                let space = ascii.iter().filter(|&x| x == " ").count();
                if space > highest_space {
                    highest_space = space;
                    found = ascii.join("");
                    found_key = key;
                }
            }
        }
    }
    // println!("{:?}", found);
    // println!("{:?}", found_key);
    println!("{:?}", found.chars().map(|x| x as usize).fold(0, |a, b| a + b));
}
