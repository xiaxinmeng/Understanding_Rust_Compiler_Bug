plain

---- [ui] src/test/ui/resolve/resolve-self-in-impl.rs stdout ----
diff of stderr:

+ error[E0391]: cycle detected when computing type of `<impl at $DIR/resolve-self-in-impl.rs:16:1: 16:10>`
+    |
+ LL | impl Self {}
+    |      ^^^^
+    |
+    |
+    = note: ...which immediately requires computing type of `<impl at $DIR/resolve-self-in-impl.rs:16:1: 16:10>` again
+    = note: cycle used when finding all inherent impls defined in crate
+ 
+ error[E0391]: cycle detected when computing type of `<impl at $DIR/resolve-self-in-impl.rs:17:1: 17:13>`
+    |
+    |
+ LL | impl S<Self> {}
+    |
+    |
+    = note: ...which immediately requires computing type of `<impl at $DIR/resolve-self-in-impl.rs:17:1: 17:13>` again
+    = note: cycle used when finding all inherent impls defined in crate
+ 
1 error[E0391]: cycle detected when computing type of `<impl at $DIR/resolve-self-in-impl.rs:14:1: 14:17>`
3    |

24    |               ^^^^
25    |
25    |
26    = note: ...which immediately requires computing type of `<impl at $DIR/resolve-self-in-impl.rs:15:1: 15:20>` again
- note: cycle used when collecting item types in top-level module
-   --> $DIR/resolve-self-in-impl.rs:1:1
-    |
- LL | / #![feature(associated_type_defaults)]
- LL | |
- LL | | struct S<T = u8>(T);
- LL | | trait Tr<T = u8> {
- LL | |
- LL | | fn main() {}
-    | |____________^
- 
- 
- error[E0391]: cycle detected when computing type of `<impl at $DIR/resolve-self-in-impl.rs:16:1: 16:10>`
-   --> $DIR/resolve-self-in-impl.rs:16:6
- LL | impl Self {}
-    |      ^^^^
-    |
-    |
-    = note: ...which immediately requires computing type of `<impl at $DIR/resolve-self-in-impl.rs:16:1: 16:10>` again
- note: cycle used when collecting item types in top-level module
-   --> $DIR/resolve-self-in-impl.rs:1:1
-    |
- LL | / #![feature(associated_type_defaults)]
- LL | |
- LL | | struct S<T = u8>(T);
- LL | | trait Tr<T = u8> {
- LL | |
- LL | | fn main() {}
-    | |____________^
- 
- 
- error[E0391]: cycle detected when computing type of `<impl at $DIR/resolve-self-in-impl.rs:17:1: 17:13>`
-   --> $DIR/resolve-self-in-impl.rs:17:8
-    |
- LL | impl S<Self> {}
-    |
-    |
-    = note: ...which immediately requires computing type of `<impl at $DIR/resolve-self-in-impl.rs:17:1: 17:13>` again
65 note: cycle used when collecting item types in top-level module
67    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-self-in-impl/resolve-self-in-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/resolve-self-in-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/resolve-self-in-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-self-in-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-self-in-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:16:1: 16:10>`
   |
   |
LL | impl Self {} //~ ERROR cycle detected
   |
   |
   = note: ...which immediately requires computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:16:1: 16:10>` again
   = note: cycle used when finding all inherent impls defined in crate

error[E0391]: cycle detected when computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:17:1: 17:13>`
   |
   |
LL | impl S<Self> {} //~ ERROR cycle detected
   |
   |
   = note: ...which immediately requires computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:17:1: 17:13>` again
   = note: cycle used when finding all inherent impls defined in crate

error[E0391]: cycle detected when computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:14:1: 14:17>`
   |
   |
LL | impl Tr for Self {} //~ ERROR cycle detected
   |
   |
   = note: ...which immediately requires computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:14:1: 14:17>` again
note: cycle used when collecting item types in top-level module
   |
   |
LL | / #![feature(associated_type_defaults)]
LL | |
LL | | struct S<T = u8>(T);
LL | | trait Tr<T = u8> {
LL | |
LL | | fn main() {}
   | |____________^


error[E0391]: cycle detected when computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:15:1: 15:20>`
   |
   |
LL | impl Tr for S<Self> {} //~ ERROR cycle detected
   |
   |
   = note: ...which immediately requires computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:15:1: 15:20>` again
note: cycle used when collecting item types in top-level module
   |
   |
LL | / #![feature(associated_type_defaults)]
LL | |
LL | | struct S<T = u8>(T);
LL | | trait Tr<T = u8> {
LL | |
LL | | fn main() {}
   | |____________^


error[E0391]: cycle detected when computing trait implemented by `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:18:1: 18:23>`
   |
   |
LL | impl Tr<Self::A> for S {} //~ ERROR cycle detected
   |
   |
   = note: ...which immediately requires computing trait implemented by `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:18:1: 18:23>` again
note: cycle used when collecting item types in top-level module
   |
   |
LL | / #![feature(associated_type_defaults)]
LL | |
LL | | struct S<T = u8>(T);
LL | | trait Tr<T = u8> {
LL | |
LL | | fn main() {}
   | |____________^

