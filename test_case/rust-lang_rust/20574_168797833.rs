
#![deny(unreachable_code)]

fn main() {
    let uc = || panic!();
    uc();
    println!("unreachable");
}
