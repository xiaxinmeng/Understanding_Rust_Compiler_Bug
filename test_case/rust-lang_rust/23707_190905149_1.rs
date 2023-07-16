 rust
// workaround
trait ScalarMarker { }
impl<A, S, B> Add<B> for Array<S>
    where A: Clone + Add<B, Output=A>,
          S: DerefMut<Target=[A]>,
          B: ScalarMarker
{
    type Output = Self;
    fn add(self, rhs: B) -> Self {
        panic!()
    }
}
