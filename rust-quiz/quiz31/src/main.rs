trait Or {
    fn f(self);
}

struct T;

impl Or for &T {
    fn f(self) {
        print!("1");
    }
}

impl Or for &&&&T {
    fn f(self) {
        print!("2");
    }
}
// 方法查找顺序
fn main() {
    let t = T;
    let wt = &T;
    let wwt = &&T;
    let wwwt = &&&T;
    let wwwwt = &&&&T;
    let wwwwwt = &&&&&T;
    t.f(); //-> &f
    wt.f(); // &f
    wwt.f(); // &&f -> &&&f -> &mut &&f -> &f
    wwwt.f(); //&&&f -> &&&&f
    wwwwt.f(); //&&&&f
    wwwwwt.f(); //&&&&&f -> &&&&&&f -> &mut &&&&&f -> &&&&f
}
