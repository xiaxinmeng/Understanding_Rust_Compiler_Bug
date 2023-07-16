 rust
pub enum F { F1(u32), F2(str), }
fn main() {
    println!("size &F: {}", ::std::mem::size_of::<&F>());
}
