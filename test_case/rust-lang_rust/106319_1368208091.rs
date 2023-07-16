plain
diff of stderr:

25    |                        +++
26 
27 error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
-   --> $DIR/type-check-2-2.rs:30:29
29    |
29    |
30 LL |         let v: Vec<u64> = vec![0, 1, 2];
-    |             - help: consider changing this to be mutable: `mut v`
+    |             |
+    |             not mutable
+    |             help: consider changing this to be mutable: `mut v`
+    |             help: consider changing this to be mutable: `mut v`
32 LL |         asm!("{}", in(reg) v[0]);
33 LL |         asm!("{}", out(reg) v[0]);
-    |                             ^ cannot borrow as mutable
- 
- error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
-   --> $DIR/type-check-2-2.rs:32:31
-    |
- LL |         let v: Vec<u64> = vec![0, 1, 2];
-    |             - help: consider changing this to be mutable: `mut v`
+    |                             - cannot borrow as mutable
+ LL |
+ LL |
42 LL |         asm!("{}", inout(reg) v[0]);
-    |                               ^ cannot borrow as mutable
+    |                               - cannot borrow as mutable
- error: aborting due to 4 previous errors
+ error: aborting due to 3 previous errors
46 
47 Some errors have detailed explanations: E0381, E0596.
47 Some errors have detailed explanations: E0381, E0596.
48 For more information about an error, try `rustc --explain E0381`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2-2/type-check-2-2.stderr
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
To only update this specific test, also pass `--test-args asm/aarch64/type-check-2-2.rs`

error: 1 errors occurred comparing output.
error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/type-check-2-2.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0381]: used binding `x` isn't initialized
   |
LL |         let x: u64;
   |             - binding declared here but left uninitialized
   |             - binding declared here but left uninitialized
LL |         asm!("{}", in(reg) x);
   |                            ^ `x` used here but it isn't initialized
help: consider assigning a value
   |
LL |         let x: u64 = 0;
   |                    +++
   |                    +++

error[E0381]: used binding `y` isn't initialized
   |
LL |         let mut y: u64;
LL |         let mut y: u64;
   |             ----- binding declared here but left uninitialized
LL |         asm!("{}", inout(reg) y);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^ `y` used here but it isn't initialized
help: consider assigning a value
   |
LL |         let mut y: u64 = 0;
   |                        +++
   |                        +++

error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
   |
   |
LL |         let v: Vec<u64> = vec![0, 1, 2];
   |             |
   |             not mutable
   |             help: consider changing this to be mutable: `mut v`
   |             help: consider changing this to be mutable: `mut v`
LL |         asm!("{}", in(reg) v[0]);
LL |         asm!("{}", out(reg) v[0]);
   |                             - cannot borrow as mutable
LL |         //~^ ERROR cannot borrow `v` as mutable, as it is not declared as mutable
LL |         asm!("{}", inout(reg) v[0]);
   |                               - cannot borrow as mutable
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0381, E0596.
For more information about an error, try `rustc --explain E0381`.
