Rust
#[windows_subsystem = "windows"]  // Just on main, not a crate level attribute (# vs #!)
fn main() {
    println!("Works fine with cargo test!");
}

#[cfg(test)]
mod main_tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
