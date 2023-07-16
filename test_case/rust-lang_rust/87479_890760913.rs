
struct Ptr<'a, A> {
  id: usize,
  marker: PhantomData<&'a A>
