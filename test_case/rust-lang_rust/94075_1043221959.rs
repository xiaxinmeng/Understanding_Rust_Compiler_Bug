plain
   Compiling chalk-engine v0.76.0
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 96 elements, found one with 104 elements
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     |
    ::: compiler/rustc_ast/src/ast.rs:1129:1
    ::: compiler/rustc_ast/src/ast.rs:1129:1
     |
1129 |   rustc_data_structures::static_assert_size!(Expr, 96);

error[E0308]: mismatched types
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
     |
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 104 elements, found one with 112 elements
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     |
    ::: compiler/rustc_ast/src/ast.rs:2767:1
    ::: compiler/rustc_ast/src/ast.rs:2767:1
     |
2767 |   rustc_data_structures::static_assert_size!(ItemKind, 104);

error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
    |
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 12 elements, found one with 16 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    |
   ::: compiler/rustc_ast/src/token.rs:246:1
   ::: compiler/rustc_ast/src/token.rs:246:1
    |
246 |   rustc_data_structures::static_assert_size!(TokenKind, 12);

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_ast` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
