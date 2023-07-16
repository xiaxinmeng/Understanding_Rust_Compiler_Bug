rust
struct Arr<T, const N: usize>
where T: Sized,
      Assert::<{N <= (usize::max_value() / std::mem::size_of::<T>())}>: IsTrue,
{
    data: [T; N],
}

enum Assert<const COND: bool> {}

trait IsTrue {}

impl IsTrue for Assert<true> {}
