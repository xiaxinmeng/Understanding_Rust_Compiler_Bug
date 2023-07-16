
[00:32:39] error[E0308]: mismatched types
[00:32:39]    --> /checkout/src/librustdoc/test.rs:127:56
[00:32:39]     |
[00:32:39] 127 |         let map = hir::map::map_crate(&mut hir_forest, defs);
[00:32:39]     |                                                        ^^^^ expected reference, found struct `rustc::hir::map::Definitions`
[00:32:39]     |
[00:32:39]     = note: expected type `&rustc::hir::map::Definitions`
[00:32:39]                found type `rustc::hir::map::Definitions`
[00:32:39]     = help: try with `&defs`
