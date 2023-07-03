struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        print!("1");
    }
}
fn main() {
    let _guard = Guard; // 4 drop
    print!("3"); // 1
    let _ = Guard; // 2 drop
    print!("2"); // 3
}
