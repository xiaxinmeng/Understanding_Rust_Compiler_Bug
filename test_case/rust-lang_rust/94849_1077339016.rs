plain
    Checking clippy_lints v0.1.61 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/loops/needless_range_loop.rs:63:73
   |
63 |                     if region_scope_tree.is_subscope_of(indexed_extent, pat_extent) {
   |                                                                         ^^^^^^^^^^ expected struct `rustc_middle::middle::region::Scope`, found enum `std::option::Option`
   = note: expected struct `rustc_middle::middle::region::Scope`
                found enum `std::option::Option<rustc_middle::middle::region::Scope>`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/loops/needless_range_loop.rs:269:39
    |
269 | ...                   (Some(extent), self.cx.typeck_results().node_type(seqexpr.hir_id)),
    |                             ^^^^^^ expected struct `rustc_middle::middle::region::Scope`, found enum `std::option::Option`
    = note: expected struct `rustc_middle::middle::region::Scope`
                 found enum `std::option::Option<rustc_middle::middle::region::Scope>`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/loops/needless_range_loop.rs:272:96
    |
272 | ...                   self.indexed_indirectly.insert(seqvar.segments[0].ident.name, Some(extent));
    |                                                                                          ^^^^^^ expected struct `rustc_middle::middle::region::Scope`, found enum `std::option::Option`
    = note: expected struct `rustc_middle::middle::region::Scope`
                 found enum `std::option::Option<rustc_middle::middle::region::Scope>`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/shadow.rs:159:31
    |
159 |     scope_tree.is_subscope_of(second_scope, first_scope)
    |                               ^^^^^^^^^^^^ expected struct `rustc_middle::middle::region::Scope`, found enum `std::option::Option`
    = note: expected struct `rustc_middle::middle::region::Scope`
                 found enum `std::option::Option<rustc_middle::middle::region::Scope>`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/shadow.rs:159:45
    |
159 |     scope_tree.is_subscope_of(second_scope, first_scope)
    |                                             ^^^^^^^^^^^ expected struct `rustc_middle::middle::region::Scope`, found enum `std::option::Option`
    = note: expected struct `rustc_middle::middle::region::Scope`
                 found enum `std::option::Option<rustc_middle::middle::region::Scope>`

For more information about this error, try `rustc --explain E0308`.
