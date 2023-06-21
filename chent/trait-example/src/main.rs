use std::fmt;

//struct 可以derive Default，但我们需要所有字段都实现Default
#[derive(Default,Clone,Debug)]
struct Developer {
name: String,
age: u8,
lang:Language,
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
enum Language {
    Rust,
    TypeScript,
    Exlixir,
    Haskell,
}

//手工实现Default
impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}

impl Developer {
    pub fn new(name:&str) -> Self {
        // 用..Default::default()为剩余字段使用缺省值
        Self { name: name.to_owned(), 
        ..Default::default() }
    }
}

impl fmt::Display for Developer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({} years old): {:?} developer",
            self.name,
                        self.age,
                        self.lang
        )
    }
}


fn main() {
    //使用T::default()
    let dev1 = Developer::default();
    //使用Default::default(),但此时类型无法通过上下文推断，需要提供类型
    let dev2: Developer = Default::default();
    //使用T::new
    let dev3 = Developer::new("Tyr");
    println!("dev1: {} \\n dev2: {}\\n dev3: {:?}", dev1, dev2,dev3);
}
