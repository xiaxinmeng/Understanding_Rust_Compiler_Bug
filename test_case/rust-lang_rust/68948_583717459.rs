rust
pub fn main() {
    let mut s = "s".to_owned();
    let a = &mut s;
    *a += "";
    let b: _ = &mut *a;
    *a += "";
}


pub fn main_2() {
    let mut s = "s".to_owned();
    let a = &mut s;
    *a += "";
    let b: &mut _ = a;
    *a += "";
}
