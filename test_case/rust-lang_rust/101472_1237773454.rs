plain
    Checking clippy_lints v0.1.65 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/excessive_bools.rs:165:43
    |
138 |         match &item.kind {
    |               ---------- this expression has type `&rustc_ast::ItemKind`
...
163 |                 of_trait: None, items, ..
    |                                 ----- first introduced with type `&std::boxed::Box<[P<rustc_ast::Item<rustc_ast::AssocItemKind>>]>` here
164 |             })
165 |             | ItemKind::Trait(box Trait { items, .. }) => {
    |                                           ^^^^^ expected struct `std::boxed::Box`, found struct `std::vec::Vec`
    |
    = note: expected reference `&std::boxed::Box<[P<rustc_ast::Item<rustc_ast::AssocItemKind>>]>`
               found reference `&std::vec::Vec<P<rustc_ast::Item<rustc_ast::AssocItemKind>>>`
    = note: in the same arm, a binding must have the same type in all alternatives

error[E0277]: `&std::boxed::Box<[P<rustc_ast::Item<rustc_ast::AssocItemKind>>]>` is not an iterator
    |
166 |                 for item in items {
166 |                 for item in items {
    |                             ^^^^^ `&std::boxed::Box<[P<rustc_ast::Item<rustc_ast::AssocItemKind>>]>` is not an iterator
    |
    = help: the trait `Iterator` is not implemented for `&std::boxed::Box<[P<rustc_ast::Item<rustc_ast::AssocItemKind>>]>`
    = help: the trait `Iterator` is implemented for `std::boxed::Box<I, A>`
    = note: required because of the requirements on the impl of `IntoIterator` for `&std::boxed::Box<[P<rustc_ast::Item<rustc_ast::AssocItemKind>>]>`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/excessive_bools.rs:168:47
    |
    |
168 |                         self.check_fn_sig(cx, sig, item.span);
    |                              |                |
    |                              |                |
    |                              |                expected `&rustc_ast::FnSig`, found struct `rustc_ast::FnSig`
    |                              |                help: consider borrowing here: `&sig`
    |
note: associated function defined here
   --> src/tools/clippy/clippy_lints/src/excessive_bools.rs:95:8
    |
    |
95  |     fn check_fn_sig(&self, cx: &EarlyContext<'_>, fn_sig: &FnSig, span: Span) {


error[E0277]: `&std::boxed::Box<[rustc_ast::GenericParam]>` is not an iterator
   --> src/tools/clippy/clippy_lints/src/misc_early/mod.rs:335:22
    |
335 |         for param in &gen.params {
    |                      ^^^^^^^^^^^ `&std::boxed::Box<[rustc_ast::GenericParam]>` is not an iterator
    |
    = help: the trait `Iterator` is not implemented for `&std::boxed::Box<[rustc_ast::GenericParam]>`
    = help: the trait `Iterator` is implemented for `std::boxed::Box<I, A>`
    = note: required because of the requirements on the impl of `IntoIterator` for `&std::boxed::Box<[rustc_ast::GenericParam]>`

error[E0599]: no method named `as_slice` found for struct `std::boxed::Box<[rustc_ast::Stmt]>` in the current scope
   |
   |
99 |         self.is_break = match block.stmts.as_slice() {
   |                                           ^^^^^^^^ method not found in `std::boxed::Box<[rustc_ast::Stmt]>`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
1  | use core::slice::SlicePattern;
1  | use core::slice::SlicePattern;
   |

error[E0277]: `&std::boxed::Box<[rustc_ast::Stmt]>` is not an iterator
   --> src/tools/clippy/clippy_lints/src/suspicious_operation_groupings.rs:386:25
    |
386 |             for stmt in &block.stmts {
    |                         ^^^^^^^^^^^^ `&std::boxed::Box<[rustc_ast::Stmt]>` is not an iterator
    |
    = help: the trait `Iterator` is not implemented for `&std::boxed::Box<[rustc_ast::Stmt]>`
    = help: the trait `Iterator` is implemented for `std::boxed::Box<I, A>`
    = note: required because of the requirements on the impl of `IntoIterator` for `&std::boxed::Box<[rustc_ast::Stmt]>`
Some errors have detailed explanations: E0277, E0308, E0599.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `clippy_lints` due to 6 previous errors
Build completed unsuccessfully in 0:03:15
