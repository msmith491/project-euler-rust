#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

fn main() {

    lazy_static! {
        static ref SVAL: HashMap<&'static str, usize> = {
            let mut hm = HashMap::new();
            hm.insert("S", 4);
            hm.insert("H", 3);
            hm.insert("D", 2);
            hm.insert("C", 1);
            hm
        };

        static ref FVAL: HashMap<&'static str, usize> = {
            let mut hm = HashMap::new();
            hm.insert("2", 1);
            hm.insert("3", 2);
            hm.insert("4", 3);
            hm.insert("5", 4);
            hm.insert("6", 5);
            hm.insert("7", 6);
            hm.insert("8", 7);
            hm.insert("9", 8);
            hm.insert("10", 9);
            hm.insert("J", 10);
            hm.insert("Q", 11);
            hm.insert("K", 12);
            hm.insert("A", 13);
            hm
        };
    }

    fn toTup(card: &str) -> (String, String) {
        let mut chars = card.chars();
        (chars.next().unwrap().to_string(),
         chars.next().unwrap().to_string())
    }

    fn getCardValue(card: (&str, &str)) -> usize {
        let f = FVAL.get(card.0).unwrap();
        let s = SVAL.get(card.1).unwrap();
        f * s
    }

    fn isFlush(hand: Vec<&str>) {
        let mut hand_copy = hand.clone();
        let card = hand_copy.iter().next().unwrap();
        let suit = card.as_bytes()[1];
        hand_copy
            .all(|&x| o
    }

    // println!("{:?}", getCardValue(("A", "S")));
    let mut f = File::open("poker.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);

    let hands = s
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|&x| (x[..(x.len()/2)].split(" ").filter(|&x| x != "").collect::<Vec<_>>(),
                   x[(x.len()/2)..].split(" ").filter(|&x| x != "").collect::<Vec<_>>()))
        .filter(|x| x.0.len() > 0)
        .collect::<Vec<_>>();

    println!("{:?}", isFlush(hands[0].clone().0));
}
