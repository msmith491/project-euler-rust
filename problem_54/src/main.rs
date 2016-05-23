#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use std::char;

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
            hm.insert("T", 9);
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

    fn getCardValue(card: (String, String)) -> usize {
        let f = FVAL.get(card.0.as_str()).unwrap();
        let s = SVAL.get(card.1.as_str()).unwrap();
        f * s
    }

    fn isFlush(hand: Vec<&str>) -> bool {
        let mut hand_copy = hand.clone();
        let card = hand_copy.iter().next().unwrap();
        let suit = card.as_bytes()[1];
        hand_copy
            .iter()
            .all(|&x| x.as_bytes()[1] == suit)
    }

    fn isStraight(hand: Vec<&str>) -> bool {
        let mut v = hand
            .iter()
            .map(|x| FVAL.get(toTup(x).0.as_str()).unwrap())
            .collect::<Vec<_>>();
        v.sort();
        for (index, elem) in v.iter().enumerate() {
            if index == v.len() - 1 {
                break;
            }
            if v[index + 1] - **elem > 1 {
                return false;
            }
        }
        true
    }

    fn isStraightFlush(hand: Vec<&str>) -> bool {
        if isFlush(hand.clone()) && isStraight(hand) {
            return true;
        }
        false
    }

    fn _pairCount(hand: Vec<&str>, goal: usize, two_pair: bool) -> bool {
        let hand_copy = hand.clone();
        let mut v = hand_copy
            .iter()
            .map(|x| toTup(x))
            .collect::<Vec<_>>();
        for card1 in hand {
            let mut count = 0;
            for f in &v {
                let c = toTup(card1);
                if c.0 == (*f).0 && c != *f {
                    count += 1;
                    if count == goal {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn isFourKind(hand: Vec<&str>) -> bool {
        _pairCount(hand, 3, false)
    }

    fn isThreeKind(hand: Vec<&str>) -> bool {
        _pairCount(hand, 2, false)
    }

    fn isPair(hand: Vec<&str>) -> bool {
        _pairCount(hand, 1, false)
    }

    fn isTwoPair(hand: Vec<&str>) -> bool {
        _pairCount(hand, 1, true)
    }

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

    println!("{:?}", _pairCount(vec!["9D", "9C", "QC", "KS", "9H"], 3));
}
