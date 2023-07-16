rust
#[target_feature = "+sse"] fn foo(f32x8) -> f32x8;
static mut foo_ptr: fn(f32x8) -> f32x8 = foo;

unsafe {
  // foo_ptr = foo_avx; // ERROR: mismatched ABI
  foo_ptr = foo_avx as fn(f32x8) -> f32x8; // OK
}

// assert_eq!(foo_ptr, foo_avx); // ERROR: mismatched ABIs
assert_eq!(foo_ptr, foo_avx as fn(f32x8) -> f32x8); //  OK
