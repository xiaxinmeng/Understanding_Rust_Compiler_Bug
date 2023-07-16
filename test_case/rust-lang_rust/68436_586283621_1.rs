Rust
struct Arr<T, const N: usize>
where T: Sized,
      (N == 0),
{
    data: [T; {32 / N}], // D:
}
