fn main() {

    println!("{}", (0..1000)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |acc, item| acc + item));

}
