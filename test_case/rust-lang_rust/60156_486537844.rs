
[01:38:01] error[E0080]: constant evaluation error: tried to call a function with ABI RustIntrinsic using caller ABI PlatformIntrinsic
[01:38:01]    --> /checkout/src/libcore/../stdsimd/crates/core_arch/src/x86/sse2.rs:820:27
[01:38:01]     |
[01:38:01] 820 |     transmute::<i8x16, _>(simd_eq(a.as_i8x16(), b.as_i8x16()))
[01:38:01]     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ tried to call a function with ABI RustIntrinsic using caller ABI PlatformIntrinsic
[01:38:01]     |
[01:38:01]     = note: inside call to `std::arch::x86_64::_mm_cmpeq_epi8` at /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.3.0/src/raw/sse2.rs:83:23
[01:38:01]     = note: inside call to `hashbrown::raw::imp::Group::match_byte` at /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.3.0/src/raw/mod.rs:853:28
[01:38:01]     = note: inside call to `hashbrown::raw::RawTable::<(i32, i32)>::find::<[closure@DefId(8/1:34 ~ hashbrown[269a]::map[0]::{{impl}}[2]::insert[0]::{{closure}}[0]) 0:&i32]>` at /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.3.0/src/map.rs:844:33
[01:38:01]     = note: inside call to `hashbrown::map::HashMap::<i32, i32, std::collections::hash_map::RandomState>::insert` at /checkout/src/libstd/collections/hash/map.rs:821:9
[01:38:01] note: inside call to `std::collections::HashMap::<i32, i32>::insert` at $DIR/hashmap.rs:7:5
[01:38:01]    --> $DIR/hashmap.rs:7:5
[01:38:01]     |
[01:38:01] 7   |     map.insert(0, 0);
[01:38:01]     |     ^^^^^^^^^^^^^^^^
