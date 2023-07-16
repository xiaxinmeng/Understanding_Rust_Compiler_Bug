plain
test [ui] src/test/ui/rfc-2497-if-let-chains/ast-pretty-check.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/chains-without-let.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/feature-gate.rs ... ok
test [ui] src/test/ui/rfc-2294-if-let-guard/run-pass.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/invalid-let-in-a-valid-let-context.rs ... ok
test [ui] src/test/ui/rfc-2091-track-caller/tracked-closure.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/irrefutable-lets.rs#allowed ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/issue-90722.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/issue-93150.rs ... ok
---

---- [ui] src/test/ui/asm/aarch64/type-check-2-2.rs stdout ----
diff of stderr:

- error[E0381]: use of possibly-uninitialized variable: `x`
+ error[E0381]: used binding `x` isn't initialized
3    |
+ LL |         let x: u64;
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
+    |             - binding declared here but left uninitialized
4 LL |         asm!("{}", in(reg) x);
-    |                            ^ use of possibly-uninitialized `x`
+    |                            ^ `x` used here but it isn't initialized
6 
- error[E0381]: use of possibly-uninitialized variable: `y`
+ error[E0381]: used binding `y` isn't initialized
9    |
+ LL |         let mut y: u64;
+ LL |         let mut y: u64;
+    |             ----- binding declared here but left uninitialized
10 LL |         asm!("{}", inout(reg) y);
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^ use of possibly-uninitialized `y`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^ `y` used here but it isn't initialized
12 
13 error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2-2/type-check-2-2.stderr
To only update this specific test, also pass `--test-args asm/aarch64/type-check-2-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/type-check-2-2.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0381]: used binding `x` isn't initialized
   |
LL |         let x: u64;
LL |         let x: u64;
   |             - binding declared here but left uninitialized
LL |         asm!("{}", in(reg) x);
   |                            ^ `x` used here but it isn't initialized

error[E0381]: used binding `y` isn't initialized
   |
LL |         let mut y: u64;
LL |         let mut y: u64;
   |             ----- binding declared here but left uninitialized
LL |         asm!("{}", inout(reg) y);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^ `y` used here but it isn't initialized

error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
   |
   |
LL |         let v: Vec<u64> = vec![0, 1, 2];
   |             - help: consider changing this to be mutable: `mut v`
LL |         asm!("{}", in(reg) v[0]);
LL |         asm!("{}", out(reg) v[0]);
   |                             ^ cannot borrow as mutable

error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
   |
   |
LL |         let v: Vec<u64> = vec![0, 1, 2];
   |             - help: consider changing this to be mutable: `mut v`
...
LL |         asm!("{}", inout(reg) v[0]);
   |                               ^ cannot borrow as mutable
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0381, E0596.
For more information about an error, try `rustc --explain E0381`.
