plain
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
   --> compiler/rustc_mir_build/src/thir/pattern/check_match.rs:161:13
    |
157 |         let mut cx = self.new_cx(scrut.hir_id);
    |                      ---- immutable borrow occurs here
...
161 |             self.check_patterns(&arm.pat);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
...
164 |                 let tpat = self.lower_pattern(&mut cx, pat, &mut false).0;
    |                                               ------- immutable borrow later used here

error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
   --> compiler/rustc_mir_build/src/thir/pattern/check_match.rs:163:17
    |
157 |         let mut cx = self.new_cx(scrut.hir_id);
    |                      ---- immutable borrow occurs here
...
163 |                 self.check_patterns(pat);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
164 |                 let tpat = self.lower_pattern(&mut cx, pat, &mut false).0;
    |                                               ------- immutable borrow later used here
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0502`.
