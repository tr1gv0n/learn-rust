mod error;
mod pb;
mod storage;
mod service;

pub use error::KvError;
pub use pb::abi::*;
pub use storage::*;
pub use service::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
