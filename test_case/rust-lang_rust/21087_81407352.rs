 rust
trait A<T> { fn b(self, T); }
impl<T> A<T> for (){ fn b(self, _: T) {} }
macro_rules! c {
    { $d:expr, $e:expr } => ( $d.b(|| $e.b(|| ())) )
}
fn main(){ c! { (), () } }
