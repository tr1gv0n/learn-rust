struct S;

impl Drop for S {
    fn drop(&mut self) {
        print!("1");
    }
}
fn main() {
    let s = S;
    let _ = s; // do not move
    print!("2");
    //21
}
