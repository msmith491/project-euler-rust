fn main() {

    let mut vec = Vec::new();

    for num in 0..1000 {
        if num % 3 == 0 || num % 5 == 0 {
            // println!("{}", num);
            vec.push(num);
        }
    }

    println!("{}", vec.into_iter().fold(0, |acc, item| acc + item));
}
