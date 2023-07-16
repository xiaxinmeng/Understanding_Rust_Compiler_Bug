plain
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
error[E0366]: negative impls cannot be specialized
    --> compiler/rustc_ast/src/ast_traits.rs:29:13
     |
26   | / macro_rules! impl_not_ast_deref {
27   | |     ($($T:ty),+ $(,)?) => {
28   | |         $(
29   | |             impl !AstDeref for $T {}
30   | |         )+
31   | |     };
32   | | }
     | |_- in this expansion of `impl_not_ast_deref!`
     | |_- in this expansion of `impl_not_ast_deref!`
33   |
34   |   impl_not_ast_deref!(AssocItem, Expr, ForeignItem, Item, Stmt);
     |
     |
     = note: `ast::AssocItemKind` is not a generic parameter
note: use the same sequence of generic lifetime, type and const parameters as the struct definition
     |
     |
2761 | pub struct Item<K = ItemKind> {

error[E0366]: negative impls cannot be specialized
    --> compiler/rustc_ast/src/ast_traits.rs:29:13
     |
     |
26   | / macro_rules! impl_not_ast_deref {
27   | |     ($($T:ty),+ $(,)?) => {
28   | |         $(
29   | |             impl !AstDeref for $T {}
30   | |         )+
31   | |     };
32   | | }
     | |_- in this expansion of `impl_not_ast_deref!`
     | |_- in this expansion of `impl_not_ast_deref!`
33   |
34   |   impl_not_ast_deref!(AssocItem, Expr, ForeignItem, Item, Stmt);
     |
     |
     = note: `ast::ForeignItemKind` is not a generic parameter
note: use the same sequence of generic lifetime, type and const parameters as the struct definition
     |
     |
2761 | pub struct Item<K = ItemKind> {

error[E0366]: negative impls cannot be specialized
    --> compiler/rustc_ast/src/ast_traits.rs:29:13
     |
     |
26   | / macro_rules! impl_not_ast_deref {
27   | |     ($($T:ty),+ $(,)?) => {
28   | |         $(
29   | |             impl !AstDeref for $T {}
30   | |         )+
31   | |     };
32   | | }
     | |_- in this expansion of `impl_not_ast_deref!`
     | |_- in this expansion of `impl_not_ast_deref!`
33   |
34   |   impl_not_ast_deref!(AssocItem, Expr, ForeignItem, Item, Stmt);
     |
     |
     = note: `ast::ItemKind` is not a generic parameter
note: use the same sequence of generic lifetime, type and const parameters as the struct definition
     |
     |
2761 | pub struct Item<K = ItemKind> {

   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
For more information about this error, try `rustc --explain E0366`.
error: could not compile `rustc_ast` (lib) due to 3 previous errors
