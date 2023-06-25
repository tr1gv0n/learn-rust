use std::{fmt,ops::Deref,str};

const MINI_STRING_MAX_LEN: usize = 30;

// Mystring里,String有2个word，共24字节，所以它以8字节对齐
// 所以enum的tag + padding最少8字节，整个结构占32字节
// MiniString可以最多有30字节（再加上1字节长度和1字节tag），就是32字节
struct MiniString{
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();
        let mut data = [0; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self { len: len as u8, data, }
}
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // 由于生成MiniString的接口是隐藏的，它智能来自字符串，所以下面这行是安全的
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
        //也可以直接用unsafe版本
        // unsafe {str::from_utf8_unchecked(&self.data[..self.len as usize])}
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            MyString::Inline(s) => s.deref(),
            MyString::Standard(s) => s.deref(),
        }
    }
}

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        match s.len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(s.to_owned()),
            _=> Self::Inline(MiniString::new(s)),
        }
    }
}

impl fmt::Display for MyString{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}


fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len: MyString {}, MiniString {}",len1,len2);

    let s1: MyString = "hello world".into();
    let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();

    println!("s1: {:?}, s2: {:?}",s1,s2);
    println!(
        "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
    s1,
    s1.len(),
    s1.chars().count(),
    s2,
    s2.len(),
    s2.chars().count()
);

assert!(s1.ends_with("world"));
assert!(s2.starts_with("这"));
}
