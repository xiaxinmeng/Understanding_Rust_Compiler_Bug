plain

3    |
4 LL |     break rust
5    |           ^^^^ not found in this scope
- -Ztrack-diagnostics: created at compiler/rustc_resolve/src/late/diagnostics.rs:289:28
+ -Ztrack-diagnostics: created at compiler/rustc_resolve/src/late/diagnostics.rs:306:28
7 
8 error[E0268]: `break` outside of a loop

10    |
11 LL |     break rust
11 LL |     break rust
12    |     ^^^^^^^^^^ cannot `break` outside of a loop
- -Ztrack-diagnostics: created at compiler/rustc_passes/src/errors.rs:957:10
+ -Ztrack-diagnostics: created at compiler/rustc_passes/src/errors.rs:956:10
14 
15 error: internal compiler error: It looks like you're trying to break rust; would you like some ICE?

18 
18 
19 note: we would appreciate a joke overview: https://github.com/rust-lang/rust/issues/43162#issuecomment-320764675
- note: rustc 1.66.0-dev running on x86_64-pc-windows-msvc
+ note: rustc 1.66.0-nightly (8c3fe0bf7 2022-10-24) running on x86_64-unknown-linux-gnu
22 
23 error: aborting due to 3 previous errors
---
To only update this specific test, also pass `--test-args track-diagnostics/track.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/track-diagnostics/track.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/track-diagnostics/track" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "track-diagnostics" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/track-diagnostics/track/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `rust` in this scope
   |
LL |     break rust
   |           ^^^^ not found in this scope
-Ztrack-diagnostics: created at compiler/rustc_resolve/src/late/diagnostics.rs:306:28
-Ztrack-diagnostics: created at compiler/rustc_resolve/src/late/diagnostics.rs:306:28

error[E0268]: `break` outside of a loop
   |
LL |     break rust
LL |     break rust
   |     ^^^^^^^^^^ cannot `break` outside of a loop
-Ztrack-diagnostics: created at compiler/rustc_passes/src/errors.rs:956:10

error: internal compiler error: It looks like you're trying to break rust; would you like some ICE?
note: the compiler expectedly panicked. this is a feature.


note: we would appreciate a joke overview: https://github.com/rust-lang/rust/issues/43162#issuecomment-320764675
note: rustc 1.66.0-nightly (8c3fe0bf7 2022-10-24) running on x86_64-unknown-linux-gnu

error: aborting due to 3 previous errors

