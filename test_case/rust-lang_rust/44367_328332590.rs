rust
// crate A: -C target-feature=sse
fn foo() -> u8x32;

// crate B: -C target-feature=avx
fn bar(u8x32);

// crate C -C target-feature=sse
extern crate A;
extern crate B;
fn baz() {
  B::bar(A::foo());
}
