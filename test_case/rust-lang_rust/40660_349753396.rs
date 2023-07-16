rust
fn main() {
    let x: usize = 3;
    let y: &usize = &3;
    if x == y {
        println!("asdf");
    }
    if y == x {
        println!("asdf");
    }
    if 3 == &3 {
        println!("asdf");
    }
    if &3 == 3 {
        println!("asdf");
    }
}
