fn main() {

    fn gen_diags(side: usize) -> Vec<usize> {

        let mut diags: Vec<usize> = vec![1];
        let mut curr = 1;
        let mut add = 2;
        let mut count = 0;
        loop {
            if count < 4 {
                curr = curr + add;
                diags.push(curr);
                count = count + 1;
            } else {
                count = 0;
                add = add + 2;
            }
            if add >= side {
                break;
            }
        }
        diags
    }

    println!("{:?}", gen_diags(1001).iter().fold(0, |acc, item| acc + item));
}
