Rust
struct Test;
impl Drop for Test {
    fn drop(&mut self) {
        println!("World");
    }
}

fn main() {
    let _keep_alive;

    if true { //condition
        let may_exist = Test;
        _keep_alive = may_exist; // Can't let this go out of scope yet!
    }

    println!("Hello");
}
