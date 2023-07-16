rs
trait MySized { const SIZE: usize; }
impl<T: Sized> MySized for T {
  const SIZE: usize = std::mem::size_of::<Self>();
}
fn main() {
  dbg!(String::SIZE); // Fails under this PR
}
