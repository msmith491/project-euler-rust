fn main() {

    fn collatz(start: u64) -> Vec<u64> {

        let mut curr = start;
        let mut vec: Vec<u64> = vec![start];

        loop {
            if curr == 1 {
                break;
            }
            if curr % 2 == 0 {
                curr = curr / 2;
            } else {
                curr = (curr * 3) + 1;
            }
            vec.push(curr);
        }
        vec
    }


    let biggest = (1..1000000).map(|x| (collatz(x).len(), x)).max().unwrap();
    println!("{:?}", biggest.1);
}
