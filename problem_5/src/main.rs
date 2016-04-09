fn main() {

    fn even_divisible(n: i64, h: i64) -> bool {
        for elem in 2..h {
            if n % elem != 0 {
                return false;
            }
        }
        true
    }

    for elem in 2521.. {
        if elem % 20 != 0 {
            continue;
        } else if even_divisible(elem, 20) {
            println!("{}", elem);
            break;
        } else {
            if elem >= 10000000000 {
                break;
            }
        }
    }
}
