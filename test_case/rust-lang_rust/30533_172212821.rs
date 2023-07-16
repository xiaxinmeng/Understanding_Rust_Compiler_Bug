 rust
#![feature(optin_builtin_traits)]

trait Magic: Copy {}
impl Magic for .. {}
impl<T:Magic> Magic for T {}

fn copy<T: Magic>(x: T) -> (T, T) { (x, x) }

#[derive(Debug)]
struct NoClone;

fn main() {
    let (a, b) = copy(NoClone); //~ ERROR E0277
    println!("{:?} {:?}", a, b);
}
