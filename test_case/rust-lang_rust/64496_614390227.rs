
error: reached the type-length limit while instantiating `std::intrinsics::drop_in_place::...c::Vec<u8>>}]>}]>>]>, ()}]>>]>))`
   --> /home/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/ptr/mod.rs:177:1
    |
177 | / pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
178 | |     // Code here does not matter - this is replaced by the
179 | |     // real drop glue by the compiler.
180 | |     drop_in_place(to_drop)
181 | | }
    | |_^
    |
    = note: consider adding a `#![type_length_limit="1465124"]` attribute to your crate
