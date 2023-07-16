 rust
struct A<T>(T);

impl<T, Args> FnOnce<Args> for A<T>
where T: FnOnce<Args> {
    type Output = <T as FnOnce<Args>>::Output;
    fn call_once(self, args: Args) -> Self::Output { FnOnce::call_once(self.0, args) }
}
