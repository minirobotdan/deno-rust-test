#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn square_works() {
        assert_eq!(crate::square(2), 4);
    }
}

#[no_mangle]
pub extern "C" fn square(x: u32) -> u32 {
    x * x
}