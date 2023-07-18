trait Trait {
    fn p(&self);
}

impl Trait for u32 {
    fn p(&self) {
        println!("1");
    }
}

impl Trait for (i32,) {
    fn p(&self) {
        println!("2");
    }
}

impl Trait for (u32, u32) {
    fn p(&self) {
        println!("3");
    }
}

impl Trait for (i32, i32) {
    fn p(&self) {
        println!("4");
    }
}

fn main() {
    (0).p();
    (0,).p();
    (0, 0).p();
    (0, 0).p();
}
