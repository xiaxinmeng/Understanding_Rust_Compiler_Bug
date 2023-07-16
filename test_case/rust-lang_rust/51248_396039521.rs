
error[E0599]: no associated item named `INNERMOST` found for type `ty::sty::DebruijnIndex` in the current scope
    --> librustc/ty/sty.rs:1314:46
     |
1024 | / newtype_index!(DebruijnIndex
1025 | |     {
1026 | |         const INNERMOST = 0,
1027 | |     });
     | |_______- associated item `INNERMOST` not found for this
...
1314 |           self.shifted_out(to_binder.index() - Self::INNERMOST.index())
     |                                                ^^^^^^^^^^^^^^^ associated item not found in `ty::sty::DebruijnIndex`
     |
     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
