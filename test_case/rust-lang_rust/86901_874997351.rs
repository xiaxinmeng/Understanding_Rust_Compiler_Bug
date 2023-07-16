plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 18 elements, found one with 17 elements
7   | | }
    | |_- in this expansion of `static_assert_size!`
    | 
    | 
   ::: compiler/rustc_middle/src/dep_graph/dep_node.rs:288:1
    |
288 |   static_assert_size!(DepNode, 18);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
