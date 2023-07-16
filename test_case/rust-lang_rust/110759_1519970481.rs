rust
fn main() {
    let mut a = String::from("fa");

    let n = {
        let c = &mut a;
        let n = &*c; // move out of c, borrow a
        drop(c); // c no longer exists
        n
    };
    
    println!("{}", n);
}
