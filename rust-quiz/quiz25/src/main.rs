
use std::fmt::{self, Display};

struct S;
impl Display for S {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("1")
    }
}

impl Drop for S {
    fn drop(&mut self) {
        print!("2");
    }
}

fn f() ->S {
    S
}

fn main() {
    let S = f();
    print!("{}", S);
    //212
}
