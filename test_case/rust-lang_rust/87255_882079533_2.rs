
failures:

---- ../liballoc/src/lib.rs - (line 58) stdout ----
error[E0425]: cannot find value `x` in this scope
 --> ../liballoc/src/lib.rs:59:7
  |
4 | match x {}
  |       ^ not found in this scope

error: duplicate lang item in crate `alloc_miri_test`: `str_alloc`.
  |
  = note: the lang item is first defined in crate `alloc` (which `std` depends on)
  = note: first definition in `alloc` loaded from /home/hyd-dev/.cache/miri/HOST/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-fd991b19f11a4e51.rmeta
  = note: second definition in `alloc_miri_test` loaded from /somewhere/miri-test-libstd/target/miri/x86_64-unknown-linux-gnu/debug/deps/liballoc_miri_test-b48d33359c7e89e2.rmeta

error: duplicate lang item in crate `alloc_miri_test`: `slice_alloc`.
  |
  = note: the lang item is first defined in crate `alloc` (which `std` depends on)
  = note: first definition in `alloc` loaded from /home/hyd-dev/.cache/miri/HOST/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-fd991b19f11a4e51.rmeta
  = note: second definition in `alloc_miri_test` loaded from /somewhere/miri-test-libstd/target/miri/x86_64-unknown-linux-gnu/debug/deps/liballoc_miri_test-b48d33359c7e89e2.rmeta

error: duplicate lang item in crate `alloc_miri_test`: `slice_u8_alloc`.
  |
  = note: the lang item is first defined in crate `alloc` (which `std` depends on)
  = note: first definition in `alloc` loaded from /home/hyd-dev/.cache/miri/HOST/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-fd991b19f11a4e51.rmeta
  = note: second definition in `alloc_miri_test` loaded from /somewhere/miri-test-libstd/target/miri/x86_64-unknown-linux-gnu/debug/deps/liballoc_miri_test-b48d33359c7e89e2.rmeta

error: duplicate lang item in crate `alloc_miri_test`: `exchange_malloc`.
  |
  = note: the lang item is first defined in crate `alloc` (which `std` depends on)
  = note: first definition in `alloc` loaded from /home/hyd-dev/.cache/miri/HOST/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-fd991b19f11a4e51.rmeta
  = note: second definition in `alloc_miri_test` loaded from /somewhere/miri-test-libstd/target/miri/x86_64-unknown-linux-gnu/debug/deps/liballoc_miri_test-b48d33359c7e89e2.rmeta

error: duplicate lang item in crate `alloc_miri_test`: `box_free`.
  |
  = note: the lang item is first defined in crate `alloc` (which `std` depends on)
  = note: first definition in `alloc` loaded from /home/hyd-dev/.cache/miri/HOST/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-fd991b19f11a4e51.rmeta
  = note: second definition in `alloc_miri_test` loaded from /somewhere/miri-test-libstd/target/miri/x86_64-unknown-linux-gnu/debug/deps/liballoc_miri_test-b48d33359c7e89e2.rmeta

error: duplicate lang item in crate `alloc_miri_test`: `owned_box`.
  |
  = note: the lang item is first defined in crate `alloc` (which `std` depends on)
  = note: first definition in `alloc` loaded from /home/hyd-dev/.cache/miri/HOST/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-fd991b19f11a4e51.rmeta
  = note: second definition in `alloc_miri_test` loaded from /somewhere/miri-test-libstd/target/miri/x86_64-unknown-linux-gnu/debug/deps/liballoc_miri_test-b48d33359c7e89e2.rmeta

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.

failures:
    ../liballoc/src/lib.rs - (line 58)

test result: FAILED. 581 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 60.56s

error: test failed, to rerun pass '--doc'
