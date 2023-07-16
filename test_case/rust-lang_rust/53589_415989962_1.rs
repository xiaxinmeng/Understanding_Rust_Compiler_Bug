Rust
struct Cacher<T, U: Copy>
    where T: Fn(U) -> U
{
    calculation: T,
    value: Option<U>,
}
