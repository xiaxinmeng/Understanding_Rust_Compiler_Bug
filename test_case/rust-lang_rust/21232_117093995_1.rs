 rust
fn main() {
    let mut a : A; // uninitialized
    let a_ = A{val: 1};
    a.val = 2;   // Initialize a here
    println!("a_: {}", a_.val);
    /// println!("a: {}", a.val);  // error: use of possibly uninitialized variable
}
