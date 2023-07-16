rs
struct HasDrop;
impl Drop for HasDrop {
    fn drop(&mut self) {}
}
pub fn main() {
    let _ = (HasDrop,);
}
