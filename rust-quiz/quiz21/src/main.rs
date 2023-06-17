
trait Trait {
    fn f(&self);
}

impl <F: FnOnce() -> bool> Trait for F {
    fn f(&self) {
        print!("1");
    }
}

impl Trait for() {
    fn f(&self) {
        print!("2");
    }
}
fn main() {

    let x = || {(return)|| true;};
    x().f();

    let x = loop {(break)|| true;};
    x.f();

    // 以上直接return或者break
    // 以下返回ture 的闭包
    let x = || {return(|| true);};
    x().f();

    let x = loop {break(|| true);};
    x.f();

    let x = || {return|| true;};
    x().f();

    let x = loop {break|| true;};
    x.f();
    // 221111
}
