use regex::Regex;
use std::str::FromStr;
pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl Parse for u8 {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^[0-9]+").unwrap();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(0, |s| s.as_str().parse().unwrap_or(0))
        } else {
            0
        }
    }
}

impl<T> Parse for T
where
    T: FromStr + Default,
{
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^[0-9]+(\.[0-9]+)").unwrap();
        let d = || Default::default();
        if let Some(captures) = re.captures(s) {
            captures.get(0).map_or(d(), |s| s.as_str().parse(()))
        } else {
            d()
        }
    }
}

#[test]
fn parse_should_work() {
    assert_eq!(u8::parse("123abcd"), 123);
    assert_eq!(u8::parse("1234abcd"), 0);
    assert_eq!(u8::parse("abcd"), 0);
    assert_eq!(f64::parse("123.45abcd"), 123.45);
    assert_eq!(f64::parse("abcd"), 0f64);
}
fn main() {
    println!("result: {}", u8::parse("255 hello world"));
}
