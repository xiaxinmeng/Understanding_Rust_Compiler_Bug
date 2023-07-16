rust
pub fn main() {
    let mut foo = 4;
    let oops = &mut foo as *mut _;
    let a = unsafe { &mut* oops }; // very bad
    let b = unsafe { &mut* oops }; //  much UB
    *a = 1;
    println!("{}", *b);
}
