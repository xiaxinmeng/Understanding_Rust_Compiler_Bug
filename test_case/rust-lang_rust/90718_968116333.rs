
error: Undefined Behavior: pointer to alloc1131 was dereferenced after this allocation got freed
   --> /playground/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:105:14
    |
105 |     unsafe { __rust_dealloc(ptr, layout.size(), layout.align()) }
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pointer to alloc1131 was dereferenced after this allocation got freed
    |
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
            
    = note: inside `std::alloc::dealloc` at /playground/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:105:14
    = note: inside `<std::alloc::Global as std::alloc::Allocator>::deallocate` at /playground/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:242:22
    = note: inside `<alloc::raw_vec::RawVec<u32> as std::ops::Drop>::drop` at /playground/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs:523:22
    = note: inside `std::ptr::drop_in_place::<alloc::raw_vec::RawVec<u32>> - shim(Some(alloc::raw_vec::RawVec<u32>))` at /playground/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:188:1
    = note: inside `std::ptr::drop_in_place::<std::vec::Vec<u32>> - shim(Some(std::vec::Vec<u32>))` at /playground/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:188:1
note: inside `main` at src/main.rs:9:1
   --> src/main.rs:9:1
    |
9   | }
    | ^
