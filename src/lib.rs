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

/// Prints Hello, World!
fn main() {
    println!("Hello, {name}!", name="World");
    println!("My name is {0}, {1} {0}", "Bond", "James");
    println!("Pi is roughly {0:.3}", 3.142)
}


#[no_mangle]
pub extern "C" fn square(x: u32) -> u32 {
    x * x
}