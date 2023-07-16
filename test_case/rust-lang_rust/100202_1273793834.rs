plain
    Checking hir-expand v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir-expand)
   Compiling proc-macro-test v0.0.0 (/checkout/src/tools/rust-analyzer/crates/proc-macro-test)
    Checking hir-def v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir-def)
    Checking proc-macro-srv-cli v0.0.0 (/checkout/src/tools/rust-analyzer/crates/proc-macro-srv-cli)
error[E0502]: cannot borrow `*self.body` as immutable because it is also borrowed as mutable
   --> crates/hir-def/src/body/pretty.rs:135:38
    |
135 |                     w!(self, "{}: ", self.body[*lbl].name);
    |                                      ^^^^^^^^^ immutable borrow occurs here
   ::: /checkout/library/core/src/ops/autoref.rs:45:5
    |
    |
45  |     $x.__rustc_unstable_auto_ref_mut_helper::<{ $crate::ops::autoref::UnstableMethodSeal }>()
    |     ----------------------------------------------------------------------------------------- mutable borrow occurs here
   ::: /checkout/library/core/src/macros/mod.rs:547:35
    |
    |
547 |                 let result = _dst.write_fmt($crate::format_args!($($arg)*));
    |                                   --------- mutable borrow later used by call

error[E0502]: cannot borrow `*self.body` as immutable because it is also borrowed as mutable
   --> crates/hir-def/src/body/pretty.rs:142:38
    |
142 |                     w!(self, "{}: ", self.body[*lbl].name);
    |                                      ^^^^^^^^^ immutable borrow occurs here
   ::: /checkout/library/core/src/ops/autoref.rs:45:5
    |
    |
45  |     $x.__rustc_unstable_auto_ref_mut_helper::<{ $crate::ops::autoref::UnstableMethodSeal }>()
    |     ----------------------------------------------------------------------------------------- mutable borrow occurs here
   ::: /checkout/library/core/src/macros/mod.rs:547:35
    |
    |
547 |                 let result = _dst.write_fmt($crate::format_args!($($arg)*));
    |                                   --------- mutable borrow later used by call

error[E0502]: cannot borrow `*self.body` as immutable because it is also borrowed as mutable
   --> crates/hir-def/src/body/pretty.rs:150:38
    |
150 |                     w!(self, "{}: ", self.body[*lbl].name);
    |                                      ^^^^^^^^^ immutable borrow occurs here
   ::: /checkout/library/core/src/ops/autoref.rs:45:5
    |
    |
45  |     $x.__rustc_unstable_auto_ref_mut_helper::<{ $crate::ops::autoref::UnstableMethodSeal }>()
    |     ----------------------------------------------------------------------------------------- mutable borrow occurs here
   ::: /checkout/library/core/src/macros/mod.rs:547:35
    |
    |
547 |                 let result = _dst.write_fmt($crate::format_args!($($arg)*));
    |                                   --------- mutable borrow later used by call

error[E0502]: cannot borrow `*self.body` as immutable because it is also borrowed as mutable
   --> crates/hir-def/src/body/pretty.rs:409:38
    |
409 |                     w!(self, "{}: ", self.body[*lbl].name);
    |                                      ^^^^^^^^^ immutable borrow occurs here
   ::: /checkout/library/core/src/ops/autoref.rs:45:5
    |
    |
45  |     $x.__rustc_unstable_auto_ref_mut_helper::<{ $crate::ops::autoref::UnstableMethodSeal }>()
    |     ----------------------------------------------------------------------------------------- mutable borrow occurs here
   ::: /checkout/library/core/src/macros/mod.rs:547:35
    |
    |
547 |                 let result = _dst.write_fmt($crate::format_args!($($arg)*));
    |                                   --------- mutable borrow later used by call
For more information about this error, try `rustc --explain E0502`.
For more information about this error, try `rustc --explain E0502`.
error: could not compile `hir-def` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `hir-def` due to 4 previous errors
