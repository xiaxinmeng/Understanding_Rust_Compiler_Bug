plain
test [ui] src/test/ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] src/test/ui/trait-bounds/select-param-env-instead-of-blanket.rs stdout ----


1 error: internal compiler error: no errors encountered even though `delay_span_bug` issued
2 
- error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:11 ~ select_param_env_instead_of_blanket[b1a3]::foo), const_param_did: None }) (end of phase transition to Optimized) at bb0[1]:
+ error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:11 ~ select_param_env_instead_of_blanket[b1a3]::foo), const_param_did: None }) (end of phase transition to Optimized) at bb0[0]:
4                                 encountered `Assign((_1, const 0_usize))` with incompatible types:
5                                 left-hand side has type: <IntFactory as Factory<T>>::Item
6                                 right-hand side has type: usize

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/select-param-env-instead-of-blanket/select-param-env-instead-of-blanket.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args trait-bounds/select-param-env-instead-of-blanket.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trait-bounds/select-param-env-instead-of-blanket.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/select-param-env-instead-of-blanket" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-Zmir-opt-level=3" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/select-param-env-instead-of-blanket/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:11 ~ select_param_env_instead_of_blanket[b1a3]::foo), const_param_did: None }) (end of phase transition to Optimized) at bb0[0]:
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
                                encountered `Assign((_1, const 0_usize))` with incompatible types:
                                left-hand side has type: <IntFactory as Factory<T>>::Item
                                right-hand side has type: usize
  --> /checkout/src/test/ui/trait-bounds/select-param-env-instead-of-blanket.rs:41:5
   |
LL |     let mut x: <IntFactory as Factory<T>>::Item = bar::<T>();
   |                                                   ---------- in this inlined function call
LL |     0usize
   |     ^^^^^^
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:128:36
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (83a287eea 2022-06-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib -Z mir-opt-level=3
query stack during panic:
end of query stack
------------------------------------------

