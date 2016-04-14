fn main() {

    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];

    let mut results = vec![0;201];
    results[0] = 1;

    for coin in coins {
        for index in coin..201 {
            results[index] = results[index] + results[index - coin];
        }
    }

    println!("{:?}", results.last().unwrap());

}
