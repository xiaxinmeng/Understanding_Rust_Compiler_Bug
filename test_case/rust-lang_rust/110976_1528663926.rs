plain

---- [ui] tests/ui/typeck/bad-index-due-to-nested.rs stdout ----
diff of stderr:

36 LL | fn index<'a, K, V>(map: &'a HashMap<K, V>, k: K) -> &'a V {
38 LL |     map[k]
-    |         ^
-    |         |
-    |         |
-    |         expected `&K`, found type parameter `K`
-    |         help: consider borrowing here: `&k`
+    |         ^ expected `&K`, found type parameter `K`
44    = note:   expected reference `&K`
45            found type parameter `K`

+ help: consider borrowing here
+ help: consider borrowing here
+    |
+ LL |     map[&k]
46 
47 error[E0308]: mismatched types
48   --> $DIR/bad-index-due-to-nested.rs:20:5


50 LL | fn index<'a, K, V>(map: &'a HashMap<K, V>, k: K) -> &'a V {
51    |                 - this type parameter               ----- expected `&'a V` because of return type
52 LL |     map[k]
-    |     |
-    |     |
-    |     expected `&V`, found type parameter `V`
-    |     help: consider borrowing here: `&map[k]`
+    |     ^^^^^^ expected `&V`, found type parameter `V`
58    = note:   expected reference `&'a V`
59            found type parameter `V`

+ help: consider borrowing here
+ help: consider borrowing here
+    |
+ LL |     &map[k]
60 
61 error: aborting due to 4 previous errors
62 

---
To only update this specific test, also pass `--test-args typeck/bad-index-due-to-nested.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/checkout/tests/ui/typeck/bad-index-due-to-nested.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/bad-index-due-to-nested" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/bad-index-due-to-nested/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `K: Hash` is not satisfied
  --> fake-test-src-base/typeck/bad-index-due-to-nested.rs:20:5
LL |     map[k]
LL |     map[k]
   |     ^^^ the trait `Hash` is not implemented for `K`
   |
note: required by a bound in `<HashMap<K, V> as Index<&K>>`
  --> fake-test-src-base/typeck/bad-index-due-to-nested.rs:9:8
LL |     K: Hash,
LL |     K: Hash,
   |        ^^^^ required by this bound in `<HashMap<K, V> as Index<&K>>`
help: consider restricting type parameter `K`
   |
LL | fn index<'a, K: std::hash::Hash, V>(map: &'a HashMap<K, V>, k: K) -> &'a V {


error[E0277]: the trait bound `V: Copy` is not satisfied
  --> fake-test-src-base/typeck/bad-index-due-to-nested.rs:20:5
LL |     map[k]
   |     ^^^ the trait `Copy` is not implemented for `V`
   |
   |
note: required by a bound in `<HashMap<K, V> as Index<&K>>`
  --> fake-test-src-base/typeck/bad-index-due-to-nested.rs:10:8
LL |     V: Copy,
LL |     V: Copy,
   |        ^^^^ required by this bound in `<HashMap<K, V> as Index<&K>>`
help: consider restricting type parameter `V`
   |
LL | fn index<'a, K, V: std::marker::Copy>(map: &'a HashMap<K, V>, k: K) -> &'a V {

error[E0308]: mismatched types
  --> fake-test-src-base/typeck/bad-index-due-to-nested.rs:20:9
   |
   |
LL | fn index<'a, K, V>(map: &'a HashMap<K, V>, k: K) -> &'a V {
LL |     map[k]
LL |     map[k]
   |         ^ expected `&K`, found type parameter `K`
   = note:   expected reference `&K`
           found type parameter `K`
help: consider borrowing here
   |
   |
LL |     map[&k]
   |         +

error[E0308]: mismatched types
  --> fake-test-src-base/typeck/bad-index-due-to-nested.rs:20:5
   |
LL | fn index<'a, K, V>(map: &'a HashMap<K, V>, k: K) -> &'a V {
   |                 - this type parameter               ----- expected `&'a V` because of return type
LL |     map[k]
   |     ^^^^^^ expected `&V`, found type parameter `V`
   = note:   expected reference `&'a V`
           found type parameter `V`
help: consider borrowing here
   |
