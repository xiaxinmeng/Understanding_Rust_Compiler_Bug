Rust
where (bool_expr)

// E.g.:
struct Arr<T, const N: usize>
where T: Sized,
      (N <= (usize::max_value() / size_of::<T>())),
{
    data: [T; N],
}
