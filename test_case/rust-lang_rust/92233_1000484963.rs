plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0308]: mismatched types
    --> compiler/rustc_borrowck/src/region_infer/mod.rs:2133:54
     |
2133 |             if let Some(next) = categorized_path.get(i + 1) {
     |                                                      |
     |                                                      expected reference, found `usize`
     |                                                      expected reference, found `usize`
     |                                                      help: consider borrowing here: `&(i + 1)`
     = note: expected reference `&_`
                     found type `usize`

error[E0308]: mismatched types
---

error[E0308]: mismatched types
    --> compiler/rustc_borrowck/src/region_infer/mod.rs:2169:9
     |
1966 |     ) -> BlameConstraint<'tcx> {
     |          --------------------- expected `BlameConstraint<'tcx>` because of return type
2169 |         categorized_path.remove(0)
2169 |         categorized_path.remove(0)
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `BlameConstraint`, found `bool`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_borrowck` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
