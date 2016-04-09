extern crate range_check;

use range_check::Contains;

fn main() {
    let tens = vec!["one",
                    "two",
                    "three",
                    "four",
                    "five",
                    "six",
                    "seven",
                    "eight",
                    "nine"];

    let ten = "ten";

    let teens = vec!["eleven",
                     "twelve",
                     "thirteen",
                     "fourteen",
                     "fifteen",
                     "sixteen",
                     "seventeen",
                     "eighteen",
                     "nineteen"];

    let twenty_plus = vec!["twenty",
                           "thirty",
                           "forty",
                           "fifty",
                           "sixty",
                           "seventy",
                           "eighty",
                           "ninety"];

    let hundred = "hundred";

    let thousand = "thousand";

    let and = "and";

    let get_number = |n: usize| -> String {

        if (1..10).contains(n) {
            return tens[n - 1].to_string();
        } else if n == 10 {
            return ten.to_string();
        } else if (11..20).contains(n) {
            return teens[n - 11].to_string();
        } else if (20..100).contains(n) {
            let t_digit = n / 10;
            let o_digit = n % 10;
            let mut s: String = String::new();

            if t_digit == 1 {
                s = ten.to_string();
            } else {
                s = (twenty_plus[t_digit - 2]).to_string();
            }

            if o_digit > 0 {
                s = s + " " + (tens[o_digit - 1]).clone();
            }
            return s;
        } else if (100..1000).contains(n) {
            let h_digit = n / 100;
            let t_digit = (n % 100) / 10;
            let o_digit = n % 10;
            // println!("{}, {}, {}", h_digit, t_digit, o_digit);
            let mut s: String = String::new();

            s = s + tens[h_digit - 1].clone() + " " + hundred;

            if t_digit == 0 {
                if o_digit > 0 {
                    s = s + " and " + (tens[o_digit - 1]).clone();
                }
            } else if t_digit == 1 && o_digit == 0 {
                s = s + " and " + ten.clone();
            } else if t_digit == 1 && o_digit > 0 {
                s = s + " and " + teens[o_digit - 1].clone();
            } else {
                s = s + " and " + (twenty_plus[t_digit - 2]).clone();
            }

            if o_digit > 0 && t_digit > 1 {
                s = s + " " + (tens[o_digit - 1]).clone();
            }
            return s;
        } else if n == 1000 {
            return "one thousand".to_string()
        } else {
            panic!("Something went horribly wrong");
        }
    };

    let result = (1..1001)
        .map(|x| get_number(x))
        .collect::<Vec<String>>()
        .iter()
        .map(|x| x.replace(" ", ""))
        .collect::<Vec<String>>()
        .iter()
        .map(|x| x.len())
        .fold(0, |acc, item| acc + item);


    println!("{}", result);
}
