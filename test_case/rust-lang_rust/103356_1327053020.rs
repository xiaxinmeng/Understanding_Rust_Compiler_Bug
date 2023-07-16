plain
   Compiling icu_list v1.0.0
   Compiling rustc_baked_icu_data v0.0.0 (/checkout/compiler/rustc_baked_icu_data)
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-6a1453ab4ff51aaf.so: undefined symbol: __rustc_proc_macro_decls_4eedd622934b4b9f__
   |
23 | pub use rustc_macros::newtype_index;
   |         ^^^^^^^^^^^^


error: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-6a1453ab4ff51aaf.so: undefined symbol: __rustc_proc_macro_decls_4eedd622934b4b9f__
   |
   |
11 | use rustc_macros::{Decodable, Encodable};

error: cannot determine resolution for the derive macro `Decodable`
   --> compiler/rustc_index/src/bit_set.rs:111:31
    |
    |
111 | #[derive(Eq, PartialEq, Hash, Decodable, Encodable)]
    |
    |
    = note: import resolution is stuck, try simplifying macro imports
error: cannot determine resolution for the derive macro `Encodable`
   --> compiler/rustc_index/src/bit_set.rs:111:42
    |
    |
111 | #[derive(Eq, PartialEq, Hash, Decodable, Encodable)]
    |
    |
    = note: import resolution is stuck, try simplifying macro imports
error: cannot determine resolution for the derive macro `Decodable`
    --> compiler/rustc_index/src/bit_set.rs:1570:38
     |
     |
1570 | #[derive(Clone, Eq, PartialEq, Hash, Decodable, Encodable)]
     |
     |
     = note: import resolution is stuck, try simplifying macro imports
error: cannot determine resolution for the derive macro `Encodable`
    --> compiler/rustc_index/src/bit_set.rs:1570:49
     |
     |
1570 | #[derive(Clone, Eq, PartialEq, Hash, Decodable, Encodable)]
     |
     |
     = note: import resolution is stuck, try simplifying macro imports
error: cannot determine resolution for the derive macro `Decodable`
    --> compiler/rustc_index/src/bit_set.rs:2047:38
     |
     |
2047 | #[derive(Copy, Clone, Eq, PartialEq, Decodable, Encodable)]
     |
     |
     = note: import resolution is stuck, try simplifying macro imports
error: cannot determine resolution for the derive macro `Encodable`
    --> compiler/rustc_index/src/bit_set.rs:2047:49
     |
     |
2047 | #[derive(Copy, Clone, Eq, PartialEq, Decodable, Encodable)]
     |
     |
     = note: import resolution is stuck, try simplifying macro imports
error: could not compile `rustc_index` due to 8 previous errors
Build completed unsuccessfully in 0:05:18
