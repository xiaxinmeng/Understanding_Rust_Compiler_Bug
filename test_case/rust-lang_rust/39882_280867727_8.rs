 rust
fn main() {
    let x = ();
    let y = unsafe { *(&x as *const () as *const i32) };
    // println!("{}", y);  // optional
}
