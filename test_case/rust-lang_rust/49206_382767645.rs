Rust
#![feature(optin_builtin_traits)]

struct NotSync;
impl !Sync for NotSync {}

fn main() {
    let mut u = NotSync;
    let mut v = NotSync;
    println!("{:?}", &mut u as *mut NotSync);
    println!("{:?}", &mut v as *mut NotSync);
}
