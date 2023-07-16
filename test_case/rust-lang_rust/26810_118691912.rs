 rust
trait A {
    type T;
}
impl A for [u8; 257] {
    type T = u8;
}
impl A for [u8; 1] {
    type T = u16;
}
type T = <[u8; C] as A>::T;
const C: usize = 257 as T as usize;
fn main() {}
