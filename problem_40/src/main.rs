fn main() {

    struct NumGen {
        num: Vec<char>,
        curr: usize
    }

    impl NumGen {

        fn new() -> NumGen {
            NumGen {num: vec!['1'], curr: 1}
        }

        fn next(&mut self) -> usize {
            self.curr = self.curr + 1;
            self.num.extend(self.curr.to_string().chars());
            self.curr
        }
    }
    let mut numgen = NumGen::new();


    loop {

        if numgen.num.len() >= 1000000 {
            break;
        }
        numgen.next();
    }

    let result = numgen.num[0].to_digit(10).unwrap() *
                 numgen.num[9].to_digit(10).unwrap() *
                 numgen.num[99].to_digit(10).unwrap() *
                 numgen.num[999].to_digit(10).unwrap() *
                 numgen.num[9999].to_digit(10).unwrap() *
                 numgen.num[99999].to_digit(10).unwrap() *
                 numgen.num[999999].to_digit(10).unwrap();

    println!("{:?}", result);
}
