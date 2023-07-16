 rust
fn main() {
    let mut a = A{val: 1};
    let a_ = a;
    /// a.val = 2;   // In the original example this line is uncommented
    println!("a_: {}", a_.val);
    /// println!("a: {}", a.val);  // error: use of moved value: `a.val` ?!?!
}
