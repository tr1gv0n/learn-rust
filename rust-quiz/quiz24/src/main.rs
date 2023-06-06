fn main() {
    let x: u8 = 1;
    const K: u8 = 2;

    macro_rules! m {
        () => {
            print!("{}{}", x, K);
        };
    }

    {
        let x: u8 = 3;
        const K: u8 = 4;
        m!();
    }

    // output: 14
    // 1. constant shadowing
    // 2. macro local variable
}
