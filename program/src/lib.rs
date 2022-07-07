//! A program demonstrating the transfer of lamports

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod entrypoint;
pub mod processor;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
