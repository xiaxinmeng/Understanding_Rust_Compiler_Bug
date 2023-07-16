rust
use core::ops::Index;
struct X;
impl Index<i32> for X {
    type Output = ();
    
    fn index(&self, _: i32) -> &() {
        &()
    }
}

fn main() {
    X[-1];
}
