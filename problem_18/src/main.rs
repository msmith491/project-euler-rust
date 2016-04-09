fn main() {

    let s = "3
             7 4
             2 4 6
             8 5 9 3"
             .split("\n")
             .map(|x| x.trim())
             .collect::<Vec<&str>>()
             .into_iter()
             .map(|x| x
                  .split_whitespace()
                  .collect::<Vec<&str>>()
                  .into_iter()
                  .map(|y| y
                       .trim()
                       .parse()
                       .unwrap())
                  .collect::<Vec<usize>>())
             .collect::<Vec<Vec<usize>>>();

    // println!("{:?}", s);
    for i in 0..(s.len()) {
        println!("{:?}", s[i]);
    }
}
