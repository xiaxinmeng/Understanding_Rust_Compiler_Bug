rust
#[rustc_layout_scalar_valid_range_start(1)]
struct S<T> {
    i: u8,
    t: T,
}
struct A((u32, u32));
static Ca: S<A>;
