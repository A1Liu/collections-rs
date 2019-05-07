#![no_std]

mod generic_containers;
mod interfaces;

pub use generic_containers::*;
pub use interfaces::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
