rs
fn call_it(f: impl FnOnce()) {}

#[target_feature(enable = "simd128")]
fn foo_simd128() {}

fn main() {
    foo_simd128(); // OK
    call_it(foo_simd128); // error: the trait `FnOnce<()>` is not implemented
}
