use std::mem;
fn main() {
    let a;
    let a = a = true;

    println!("{}",mem::size_of_val(&a));
    // 0
    // 赋值表达式的值始终为()
}
