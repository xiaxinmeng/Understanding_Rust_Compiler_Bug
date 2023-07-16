rs
const TOOLCHAIN : &str = include_str!("../rust-toolchain");

fn main() {
    println!("does not matter in exe, but toolchain is: {}", TOOLCHAIN);

    println!("In exe `std::collections::HashSet::new()` works");
    let mut has_set = std::collections::HashSet::new();

    println!("No issue on Windows 7");
    has_set.insert(0);

    unsafe{is_dll_ok()};
    println!("dll called");
}

#[link(name = "test_dll")]
extern "cdecl" {
    pub fn is_dll_ok();
}

