Rust
fn foo<'x>(mut x: (&'x isize, ())) {
    let a = 1;
    let (mut _z, ref _y) = x;
    _z = &a; //~ ERROR no subtyping for you!
}
      
fn main() {}
