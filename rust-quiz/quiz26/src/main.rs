fn main() {
    let input = vec![1, 2, 3];

    let parity = input.iter().map(|x| {
        println!("{}", x);
        x % 2
    });

    for p in parity {
        println!("{}", p);
    }
    // 1 1 2 0 3 1
}
